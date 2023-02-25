use std::collections::HashMap;
use std::fs::{DirEntry, File};
use std::io::Read;

use crate::logger::{self, LogLevel};
use crate::mapper;

pub fn get_chunks(files: Vec<DirEntry>) -> HashMap<String, u64> {
    let mut chunk_map = HashMap::<String, u64>::new();

    for file in files {
        let lang_name = mapper::map_file_to_lang(&file);
        if lang_name.is_empty() {
            continue;
        }

        let chunks = (get_file_len(&file) as f64) / 100.0;
        logger::log(format!("File at \"{}\" has {} chunks", file.path().display(), chunks.round() as u64), LogLevel::Info);

        if chunk_map.contains_key(&lang_name) {
            let current_chunks = chunk_map.get(&lang_name).unwrap();
            let chunks_add = chunks.round() as u64;

            chunk_map.insert(lang_name, current_chunks + chunks_add);
            continue;
        }

        chunk_map.insert(lang_name, chunks.round() as u64);
    }

    chunk_map
}

fn get_file_len(file: &DirEntry) -> usize {
    if file.metadata().unwrap().is_dir() {
        return 0;
    }
    let mut f = File::open(file.path()).expect(&format!("Could not open file: {}", file.path().display()));
    let size = f.read_to_string(&mut String::new()).unwrap_or_else(|_| { 0 });

    size
}
