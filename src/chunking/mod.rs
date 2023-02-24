use std::{collections::HashMap, fs::{DirEntry, self}};

pub fn get_chunks(langs: Vec<String>, files: Vec<DirEntry>) -> HashMap<String, u64> {
    let mut chunk_map = HashMap::<String, u64>::new();

    for file in files {
        let chunks = (get_file_len(file) as f64) / 100.0;
    }

    chunk_map
}

fn get_file_len(file: DirEntry) -> usize {
    let content = fs::read_to_string(file.path()).expect(&format!("Could not access file at: {}", file.path().display()));
    content.len()
}
