use crate::results::Scores;
use crate::indexdata::{InvertedIndex, MovieData};
use crate::idindexdata::{IdIndex};
mod results;
mod indexdata;
mod idindexdata;

use actix_web::{get, web, App, HttpRequest, HttpResponse, HttpServer};
use serde::{Deserialize, Serialize};



use std::io::Error;
use std::sync::Arc;
// This struct represents state
struct AppState {
    app_name: String,
    index_offsets: Arc<InvertedIndex>,
    data: Arc<MovieData>,
    id_index: Arc<IdIndex>
}

#[get("/")]
async fn index(data: web::Data<AppState>) -> String {
    let app_name = &data.app_name; // <- get app_name
    format!("Hello {app_name}!") // <- response with app_name
}

#[derive(Serialize, Deserialize)]
struct Query {
    query: String,
}

#[derive(Serialize, Deserialize)]
struct SimilarityQuery {
    title_id: String,
    genres: Vec<String>,
    original_query: String 
}



#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let index_offsets_original=  Arc::new(InvertedIndex::new());
    let data_original = Arc::new(MovieData::new());
    let id_index_original = Arc::new(IdIndex::new());
    HttpServer::new(move|| {
        App::new()
            .app_data(web::Data::new(AppState {
                app_name: String::from("Actix Web"),
                index_offsets: Arc::clone(&index_offsets_original),
                data: Arc::clone(&data_original),
                id_index: Arc::clone(&id_index_original)
            }))
            .service(index)
            .route("/match", web::post().to(get_matches))
            .route("/similar", web::post().to(get_similar))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

async fn get_similar(req: HttpRequest, payload: web::Json<SimilarityQuery>) -> Result<HttpResponse, Error>{
    if let Some(result) = req.app_data::<web::Data<AppState>>() {
        let query_terms: Vec<String> = payload
            .original_query
            .split_whitespace()
            .into_iter()
            .map(|term| term.to_lowercase())
            .collect();

        let mut query_scores = Scores::new();
        for word in query_terms.iter() {
            match result.index_offsets.get_matches(word){
                Ok(new_scores) => {query_scores.intersect(new_scores);}
                Err(data) => {return Ok(HttpResponse::Ok()
                    .content_type("text/plain")
                    .body(data.to_string()));}
            }
        }

        match result.id_index.get_matches(payload.title_id.to_owned()){
            Ok(scores) => {
                query_scores.intersect(scores);
            },
            Err(text) => panic!("{}", text),
        }

        query_scores.rerank(payload.genres.clone(), Arc::clone(&result.data));

        if !query_scores.empty() {
            return Ok(HttpResponse::Ok()
                .content_type("text/plain")
                .body(serde_json::to_string(&query_scores.get_top_k(10, Arc::clone(&result.data)))?));
        } else {
            return Ok(HttpResponse::Ok()
                .content_type("text/plain")
                .body("Not Found"));
        }
    } else {
        return Ok(HttpResponse::Ok()
            .content_type("text/plain")
            .body("Could not load indices"));
    }
}


async fn get_matches(req: HttpRequest, payload: web::Json<Query>) -> Result<HttpResponse, Error> {
    if let Some(result) = req.app_data::<web::Data<AppState>>() {
        let query_terms: Vec<String> = payload
            .query
            .split_whitespace()
            .into_iter()
            .map(|term| term.to_lowercase())
            .collect();

        if query_terms.len() < 1 {
            return Ok(HttpResponse::Ok()
                .content_type("text/plain")
                .body("Empty Query"));
        }
        let mut query_scores = Scores::new();
        for word in query_terms.iter() {
            match result.index_offsets.get_matches(word){
                Ok(new_scores) => {query_scores.intersect(new_scores);}
                Err(data) => {return Ok(HttpResponse::Ok()
                    .content_type("text/plain")
                    .body(data.to_string()));}
            }
        }

        if !query_scores.empty() {
            return Ok(HttpResponse::Ok()
                .content_type("text/plain")
                .body(serde_json::to_string(&query_scores.get_top_k(10, Arc::clone(&result.data)))?));
        } else {
            return Ok(HttpResponse::Ok()
                .content_type("text/plain")
                .body("Not Found"));
        }
    } else {
        return Ok(HttpResponse::Ok()
            .content_type("text/plain")
            .body("Could not load indices"));
    }
}