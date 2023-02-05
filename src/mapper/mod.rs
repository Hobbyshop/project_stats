use std::{fs::DirEntry};

pub fn map_languages(files: Vec<DirEntry>) -> Vec<String> {
    for file in files {
        let file_ending = get_file_ending(file);
        println!("{}", file_ending);
    }

    vec![String::new()]
}

fn get_file_ending(file: DirEntry) -> String {
    if !file.file_name().to_str().unwrap().contains(".") {
        return String::new();
    }

    let file_string = file.path().display().to_string();
    let mut split = file_string.split(".").collect::<Vec<&str>>();
    split.reverse();

    String::from(split[0])
}
