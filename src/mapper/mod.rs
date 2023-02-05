use std::fs::DirEntry;

use crate::logger::{self, LogLevel};

pub fn map_languages(files: Vec<DirEntry>) -> Vec<String> {
    let mut file_langs = Vec::<String>::new();

    for file in files {
        let file_ending = get_file_ending(&file);
        if file_ending.is_empty() {
            continue;
        }

        file_langs.push(file_ending);
        logger::log(format!("Registered file: \"{}\"", file.path().display()), LogLevel::Info);
    }

    logger::log(format!("Found {} valid files", file_langs.len()), LogLevel::Important);
    file_langs
}

fn get_file_ending(file: &DirEntry) -> String {
    if !file.file_name().to_str().unwrap().contains(".") {
        return String::new();
    }

    let file_string = file.path().display().to_string();
    let mut split = file_string.split(".").collect::<Vec<&str>>();
    split.reverse();

    String::from(split[0])
}
