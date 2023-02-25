mod lookup;

use std::fs::DirEntry;

pub fn map_file_to_lang(file: &DirEntry) -> String {
    let file_ending = get_file_ending(&file);
    if file_ending.is_empty() || !lookup::LANGUAGE_MAP.contains_key(file_ending.as_str()) {
        return String::new();
    }

    let lang = *lookup::LANGUAGE_MAP.get(file_ending.as_str()).unwrap();
        String::from(lang)
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
