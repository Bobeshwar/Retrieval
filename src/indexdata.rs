use crate::results::{Index, Scores};
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::io::{BufRead, BufReader, Error, Seek, SeekFrom};
use std::str;
use serde::{Deserialize,Serialize};

pub struct InvertedIndex {
    pub index_list: Vec<(String, f64, String)>,
    pub index_offsets: HashMap<String, HashMap<String, (u64, u64)>>,
}

impl InvertedIndex {
    pub fn new() -> Self {
        println!("Building inverted indexes");
        let mut output = InvertedIndex {
            index_list: Vec::new(),
            index_offsets: HashMap::new(),
        };

        output
            .index_list
            .push(("data/Index1.json".to_owned(), 1.0, "field1".to_owned()));
        output
            .index_list
            .push(("data/Index2.json".to_owned(), 5.0, "field2".to_owned()));
        output
            .index_list
            .push(("data/title.json".to_owned(), 10.0, "title".to_owned()));

        for (file_path, _weight, field) in output.index_list.iter() {
            let  f1 = File::open(file_path).unwrap();
            let mut reader = BufReader::new(f1);
            let mut buffer = Vec::<u8>::new();
            let mut bytes_so_far: u64 = 0;
            let mut offsets_map: HashMap<String, (u64, u64)> = HashMap::new();
            while let Ok(some_bytes) = reader.read_until(b'\n', &mut buffer) {
                if some_bytes != 0 {
                    match serde_json::from_slice::<Index>(&buffer) {
                        Ok(index_json) => {
                            offsets_map
                                .insert(index_json.term.clone(), (bytes_so_far, some_bytes as u64));
                        }
                        Err(content) => println!("Error {}", content),
                    }
                    bytes_so_far += some_bytes as u64;
                } else {
                    break;
                }
                buffer.clear();
            }
            output.index_offsets.insert(field.to_string(), offsets_map);
        }
        println!("Finished building inverted indexes");
        output
    }

    pub fn get_matches(&self, word: &String) -> Result<Scores, Error> {
        let mut result = Scores::new();
        for (file_path, weight, field) in self.index_list.iter() {
            let mut f1 = File::open(file_path).unwrap();
            if let Some(offsets) = self.index_offsets.get(field).unwrap().get(word) {
                let mut buffer = vec![0u8; offsets.1 as usize];
                f1.seek(SeekFrom::Start(offsets.0))?;
                f1.read_exact(&mut buffer)?;
                match serde_json::from_slice(buffer.as_slice()) {
                    Ok(index_found) => {
                        if let Some(index_json) = index_found {
                            result.update(index_json, *weight, field);
                        }
                    }
                    Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
                };
            }
        }
        Ok(result)
    }
}

#[derive(Serialize, Deserialize)]
pub struct MovieRecord {
    titletype: String,
    primarytitle: String,
    originaltitle: String,
    isadult: String,
    startyear: String,
    endyear: String,
    runtimeminutes: String,
    genres: String,
}
pub struct MovieData {
    ratings: HashMap<String, f64>,
    details: HashMap<String, (u64, usize)>,
}

#[derive(Debug, serde::Deserialize)]
pub struct RatingRecord {
    id: String,
    rating: f32,
    num: u32
}

impl MovieData {
    pub fn new() -> Self {
        println!("Building movie details database");
        let mut output = MovieData {
            ratings: HashMap::new(),
            details: HashMap::new(),
        };
        let f1 = File::open("data/TitleData.tsv").unwrap();
        let mut reader = BufReader::new(f1);
        let mut buffer = Vec::<u8>::new();
        let mut bytes_so_far: u64 = 0;
        bytes_so_far += reader.read_until(b'\n', &mut buffer).unwrap() as u64;
        buffer.clear();                                             
        while let Ok(some_bytes) = reader.read_until(b'\n', &mut buffer) {
            if some_bytes != 0 {
                let parts = str::from_utf8(&buffer).unwrap();
                let mut iterator = parts.split('\t').into_iter();
                if let Some(id) = iterator.next() {
                    output
                        .details
                        .insert(id.to_owned(), (bytes_so_far, some_bytes));
                }
                bytes_so_far += some_bytes as u64;
            } else {
                break;
            }
            buffer.clear();
        }
        println!("Building ratings database");
        let f2 = File::open("data/Ratings.tsv").unwrap();
        let mut rdr = csv::ReaderBuilder::new()
            .delimiter(b'\t')
            .from_reader(f2);
        let mut iter = rdr.deserialize();
        while let Some(result) = iter.next() {
            let record: RatingRecord = result.unwrap();
            output.ratings.insert(record.id, (record.rating as f64 - 2.5f64)* (record.num as f64).log2());
        }
        println!("Complete ratings and details database");
        output
    }

    pub fn get_movie_details( &self,id: String) -> Option<MovieRecord>{
        match self.details.get(&id) {
            None => None,
            Some(offsets) => {

                let mut f1 = File::open("data/TitleData.tsv").unwrap();
                let _ = f1.seek(SeekFrom::Start(offsets.0));
                let mut buffer = vec![0u8; offsets.1];
                let _ = f1.read_exact(&mut buffer);
                let parts: Vec<&str> = str::from_utf8(&buffer).unwrap().split("\t").into_iter().collect();
                let result = MovieRecord{
                    titletype: parts[1].to_owned(),
                    primarytitle: parts[2].to_owned(),
                    originaltitle: parts[3].to_owned(),
                    isadult: parts[4].to_owned(),
                    startyear: parts[5].to_owned(),
                    endyear: parts[6].to_owned(),
                    runtimeminutes: parts[7].to_owned(),
                    genres: parts[8].to_owned(),
                };
                Some(result)
            }
        }
    }

    pub fn get_movie_rating_score(&self, id: &String) -> f64{
        match self.ratings.get(id){
            Some(val) => *val,
            None => 0.0
        }
    }
}
