use crate::results::{Index,Scores};
mod results;


use actix_web::{get, web, App, HttpServer, HttpRequest, HttpResponse};
use serde::{Deserialize, Serialize};
use std::io::{ BufRead,BufReader, Error};
use std::fs::{File};

// This struct represents state
struct AppState {
    app_name: String,
}

#[get("/")]
async fn index(data: web::Data<AppState>) -> String {
    let app_name = &data.app_name; // <- get app_name
    format!("Hello {app_name}!") // <- response with app_name
}

#[derive(Serialize, Deserialize)]
struct Query {
    words: Vec<String>
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .app_data(web::Data::new(AppState {
                app_name: String::from("Actix Web"),
            }))
            .service(index)
            .route("/match", web::post().to(getMatches))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

async fn getMatches(req: HttpRequest,payload: web::Json<Query>) -> Result<HttpResponse,Error>{
    let file_path = "data/sampleIndex.json".to_owned();
    let queryTerms: Vec<String> = payload.words.iter().map(|term|term.to_lowercase()).collect();
    
    if queryTerms.len() < 1{
        return Ok(HttpResponse::Ok()
            .content_type("text/plain")
            .body("Empty Query"));
    }
    let mut indexList = Vec::<(String, f64, String)>::new();
    indexList.push(("data/Index1.json".to_owned(), 1.0, "field1".to_owned()));
    indexList.push(("data/Index2.json".to_owned(), 5.0, "field2".to_owned()));
    let mut queryScores = Scores::new();
    for word in queryTerms.iter(){
        let mut result = Scores::new();
        for (filePath, weight,field) in indexList.iter(){
            let indexFound: Option<Index> = getIndex(filePath, word)?;
            if let Some(indexJson) = indexFound{
                result.update(indexJson, *weight, field);    
            }
        }
        queryScores.intersect(result);

    }

    if !queryScores.empty(){
        return Ok(HttpResponse::Ok()
            .content_type("text/plain")
            .body(serde_json::to_string(&queryScores.getTopk(10))?))
    } else {
        return Ok(HttpResponse::Ok()
                .content_type("text/plain")
                .body("Not Found"));
    }
    
    
   
}

fn getIndex(filePath: &String, word: &String)  -> Result<Option<Index>, std::io::Error> {
    let file = File::open(filePath)?;
    let mut reader = BufReader::new(file);

    let mut line = String::new();
    loop {
        let bytes_read = reader.read_line(&mut line)?;
        if bytes_read == 0 {
            break;
        }
        let indexJson: Index = serde_json::from_str(&line.trim())?;
        if &indexJson.term == word{
            return Ok(Some(indexJson))
        }
        line.clear();
    }
    Ok(None)
}