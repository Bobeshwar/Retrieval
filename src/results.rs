use serde::{Serialize, Deserialize};
use std::collections::{HashMap, HashSet};
use std::cmp::Ordering;

#[derive(Serialize, Deserialize)]
pub struct Index {
    pub term: String,
    occurences: Vec<i32>,
}

pub struct Scores {
    termScores: HashMap<i32,f64>,
    documents: HashSet<i32>,
    created: bool
}


impl Scores{
    pub fn getTopk(&self, k:usize) -> Vec<i32>{
        let mut list: Vec<(i32, f64)> = Vec::new();
        for (key,val) in self.termScores.iter(){
            list.push((*key,*val));
        };
        list.sort_by(|a, b| a.1.partial_cmp(&b.1).map(Ordering::reverse).unwrap());
        let mut output:Vec<i32> = Vec::new();
        output.reserve(k);
        let mut k = k;
        if list.len() < k{
            k = list.len();
        }
        for i in 0..k{
            output.push(list[i].0);
        }
        output

    }

    pub fn new() -> Self{
        let mut result = Scores{
            termScores: HashMap::new(),
            documents: HashSet::new(),
            created: true
        };
        
        return result;
    }
    pub fn update(&mut self, newIndex: Index, weight: f64){
        for doc in newIndex.occurences.iter(){
            self.documents.insert(*doc);
            match(self.termScores.get(doc)){
                Some(score) => self.termScores.insert(*doc, *score + weight),
                None => self.termScores.insert(*doc, weight)
            };
        }
    }

    pub fn intersect(&mut self, newScores: Scores){
        if self.created{
            *self = newScores;
            self.created = false;
        } else {
            self.documents.retain(|document|newScores.documents.contains(document));
            self.termScores.retain(|document, _termScore|newScores.documents.contains(document));
            for document in self.documents.iter(){
                match(self.termScores.get(document)){
                    Some(score) => self.termScores.insert(*document, *score + newScores.termScores.get(document).unwrap()),
                    None => panic!("Term should be present")
                };
            }
        }
    }

    pub fn empty(&self) -> bool{
        self.termScores.len() == 0
    }
}