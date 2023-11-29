use serde::{Serialize, Deserialize};
use std::collections::{HashMap, HashSet};
use std::cmp::Ordering;
use crate::indexdata::{MovieData, MovieRecord};
use std::sync::Arc;

#[derive(Serialize, Deserialize)]
pub struct Index {
    pub term: String,
    documents: Vec<String>,
    document_count: i64,

}

pub struct Scores {
    termScores: HashMap<String,f64>,
    documents: HashMap<String, HashSet<String>>,
}


impl Scores{
    pub fn getTopk( mut self, k:usize,movieData: Arc<MovieData>) -> Vec<(MovieRecord, Vec<String>)>{
        let mut list: Vec<(String, f64)> = Vec::new();
        for (key,val ) in self.termScores.into_iter(){
            list.push((key.clone(),val+ movieData.getMovieRatingScore(&key)));
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
                output.push(( movieData.getMovieDetails(list[i].0.clone()).unwrap() , (matches).into_iter().collect()));
            }
            
        }
        output

    }

    pub fn new() -> Self{
        let mut result = Scores{
            termScores: HashMap::new(),
            documents: HashMap::new(),
        };
        
        return result;
    }
    pub fn update(&mut self, newIndex: Index, weight: f64, field: &String){
        for doc in newIndex.documents.into_iter(){
            self.documents.entry(doc.clone()).or_default().insert(field.to_string());
            match(self.termScores.get(&doc)){
                Some(score) => self.termScores.insert(doc, *score + weight),
                None => self.termScores.insert(doc, weight)
            };
        }
    }

    pub fn intersect(&mut self, newScores: Scores){
            // self.documents.retain(|document, _matches|newScores.documents.contains_key(document));
            // self.termScores.retain(|document, _termScore|newScores.documents.contains_key(document));
        for (document, matches) in newScores.documents.into_iter(){
            match(self.termScores.get(&document)){
                Some(score) => {self.termScores.insert(document.clone(), *score + newScores.termScores.get(&document).unwrap());},
                None => {self.termScores.insert(document.clone(), *newScores.termScores.get(&document).unwrap());}
            };
            for field in matches.iter(){ 
                self.documents.entry(document.clone()).or_default().insert(field.to_string());
            }
        }
    }

    pub fn empty(&self) -> bool{
        self.termScores.len() == 0
    }
}