use serde::{Serialize, Deserialize};
use std::collections::{HashMap, HashSet};
use std::cmp::Ordering;
use crate::indexdata::{MovieData, MovieRecord};
use std::sync::Arc;

#[derive(Serialize, Deserialize)]
pub struct Index {
    pub term: String,
    pub documents: Vec<String>,
    pub document_count: i64,

}

pub struct Scores {
    term_scores: HashMap<String,f64>,
    documents: HashMap<String, HashSet<String>>,
}


impl Scores{
    pub fn get_top_k( mut self, k:usize,movie_data: Arc<MovieData>) -> Vec<(MovieRecord, Vec<String>)>{
        let mut list: Vec<(String, f64)> = Vec::new();
        for (key,val ) in self.term_scores.into_iter(){
            list.push((key.clone(),val+ movie_data.get_movie_rating_score(&key)));
        };
        list.sort_by(|a, b| a.1.partial_cmp(&b.1).map(Ordering::reverse).unwrap());
        let mut output:Vec<(MovieRecord, Vec<String>)> = Vec::new();
        output.reserve(k);
        let mut k = k;
        if list.len() < k{
            k = list.len();
        }
        for i in 0..k{
            if let Some(matches) =  self.documents.remove(&list[i].0){
                output.push(( movie_data.get_movie_details(list[i].0.clone()).unwrap() , (matches).into_iter().collect()));
            }
            
        }
        output

    }

    pub fn new() -> Self{
        let result = Scores{
            term_scores: HashMap::new(),
            documents: HashMap::new(),
        };
        
        return result;
    }
    pub fn update(&mut self, new_index: Index, weight: f64, field: &String){
        for doc in new_index.documents.into_iter(){
            self.documents.entry(doc.clone()).or_default().insert(field.to_string());
            let new_weight: f64 = weight + (1.0f64/(new_index.document_count as f64 + 1f64).log10());
            match self.term_scores.get(&doc){
                Some(score) => self.term_scores.insert(doc, *score + new_weight),
                None => self.term_scores.insert(doc, new_weight)
            };
        }
    }

    pub fn rerank(&mut self, genres: Vec<String>){

    }

    pub fn intersect(&mut self, new_scores: Scores){
            // self.documents.retain(|document, _matches|newScores.documents.contains_key(document));
            // self.termScores.retain(|document, _termScore|newScores.documents.contains_key(document));
        for (document, matches) in new_scores.documents.into_iter(){
            match self.term_scores.get(&document){
                Some(score) => {self.term_scores.insert(document.clone(), *score + new_scores.term_scores.get(&document).unwrap());},
                None => {self.term_scores.insert(document.clone(), *new_scores.term_scores.get(&document).unwrap());}
            };
            for field in matches.iter(){ 
                self.documents.entry(document.clone()).or_default().insert(field.to_string());
            }
        }
    }

    pub fn empty(&self) -> bool{
        self.term_scores.len() == 0
    }
}