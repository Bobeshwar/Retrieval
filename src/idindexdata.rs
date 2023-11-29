use crate::indexdata::MovieRecord;
use crate::results::{Index, Scores};
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::io::{BufRead, BufReader, Error, Seek, SeekFrom};
use std::str;
use serde::{Deserialize,Serialize};

pub struct IdIndex{
    pub index_list: Vec<(String, f64, String)>,
    pub index_offsets: HashMap<String, HashMap<String, (u64, u64)>>,
    movie_metadata_offsets: HashMap<String, (u64, u64)>
}

impl IdIndexList{
    fn to_index(self) -> Index{
        return Index{
            term: self.nconst,
            documents: self.title_ids,
            document_count: self.title_count as i64,
        }
    }
}

#[derive(Serialize, Deserialize)]
struct IdIndexList{
    nconst: String,
    title_count: u64,
    title_ids: Vec<String>
}

#[derive(Serialize, Deserialize)]
struct MetaDataList{
    tconst: String,
    actors: Vec<String>,
    actresses: Vec<String>,
    directors: Vec<String>,
    producers: Vec<String>,
    writers: Vec<String>
}



impl IdIndex {
    pub fn get_matches(&self, movie_id: String) -> Result<Scores, Error>{
        let mut result = Scores::new();

        if let Some(offsets) = self.movie_metadata_offsets.get(&movie_id){
            let mut f0 = File::open("data/movie-metadata.json").unwrap();
            let mut buffer = vec![0u8; offsets.1 as usize];
            f0.seek(SeekFrom::Start(offsets.0))?;
            f0.read_exact(&mut buffer)?;
            match serde_json::from_slice::<MetaDataList>(buffer.as_slice()) {
                Ok(index_found) => {
                    let mut movieTitles = HashMap::<String, Vec<String>>::new();
                    movieTitles.insert("actor".to_owned(), index_found.actors);
                    movieTitles.insert("actress".to_owned(), index_found.actresses);
                    movieTitles.insert("director".to_owned(), index_found.directors);
                    movieTitles.insert("producer".to_owned(), index_found.producers);
                    movieTitles.insert("writer".to_owned(), index_found.writers);
                    for (file_path, weight, field) in self.index_list.iter() {
                        let mut f1 = File::open(file_path).unwrap();
                        for title in movieTitles.get(field).unwrap().into_iter(){
                            let titleoffsets = self.index_offsets.get(field).unwrap().get(title).unwrap();
                            let mut buffer = vec![0u8; titleoffsets.1 as usize];
                            f1.seek(SeekFrom::Start(titleoffsets.0))?;
                            f1.read_exact(&mut buffer)?;
                            match serde_json::from_slice::<IdIndexList>(buffer.as_slice()) {
                                Ok(index_found) => {
                                    result.update(index_found.to_index(), *weight, field);
                                    
                                }
                                Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
                            };
                        }
                    }
                    
                }
                Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
            };

        }

        
        Ok(result)
    }

    pub fn new() -> Self{
        let mut output =  IdIndex { index_list: Vec::new(), index_offsets: HashMap::new(), movie_metadata_offsets: HashMap::new() };
        println!("Building inverted id indexes");

        output.index_list.push((
            "data/actor_id_to_movies.json".to_owned(),
            10.0,
            "actor".to_owned(),
        ));
        output.index_list.push((
            "data/actress_id_to_movies.json".to_owned(),
            10.0,
            "actress".to_owned(),
        ));
        output.index_list.push((
            "data/director_id_to_movies.json".to_owned(),
            10.0,
            "director".to_owned(),
        ));
        output.index_list.push((
            "data/producer_id_to_movies.json".to_owned(),
            10.0,
            "producer".to_owned(),
        ));
        output.index_list.push((
            "data/writer_id_to_movies.json".to_owned(),
            10.0,
            "writer".to_owned(),
        ));
        for (file_path, _weight, field) in output.index_list.iter() {
            let f1 = File::open(file_path).unwrap();
            let mut reader = BufReader::new(f1);
            let mut buffer = Vec::<u8>::new();
            let mut bytes_so_far: u64 = 0;
            let mut offsets_map: HashMap<String, (u64, u64)> = HashMap::new();
            while let Ok(some_bytes) = reader.read_until(b'\n', &mut buffer) {
                if some_bytes != 0 {
                    match serde_json::from_slice::<IdIndexList>(&buffer) {
                        Ok(index_json) => {
                            offsets_map
                                .insert(index_json.nconst.to_owned(), (bytes_so_far, some_bytes as u64));
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
        println!("Finished building id indexes");
        println!("Building metadata index");
        let f1 = File::open("data/movie-metadata.json").unwrap();
        let mut reader = BufReader::new(f1);
        let mut buffer = Vec::<u8>::new();
        let mut bytes_so_far: u64 = 0;
        while let Ok(some_bytes) = reader.read_until(b'\n', &mut buffer) {
            if some_bytes != 0 {
                match serde_json::from_slice::<MetaDataList>(&buffer) {
                    Ok(index_json) => {
                        output.movie_metadata_offsets
                            .insert(index_json.tconst.to_owned(), (bytes_so_far, some_bytes as u64));
                    }
                    Err(content) => println!("Error {}", content),
                }
                bytes_so_far += some_bytes as u64;
            } else {
                break;
            }
            buffer.clear();
        };
        println!("Finished building metadata index");
        output
    }
}