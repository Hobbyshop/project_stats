mod logger;
mod mapper;

use logger::LogLevel;
use std::{fs::{self, ReadDir, DirEntry}};

fn main() {
    let args = &std::env::args().collect::<Vec<String>>();
    if args.len() == 1 {
        logger::log("Please provide a directory as program argument!".to_string(), LogLevel::Fatal);
        return;
    }

    let path = fs::read_dir(&args[1]).unwrap();
    mapper::map_languages(read_dir(path));
}

fn read_dir(dir: ReadDir) -> Vec<DirEntry> {
    let mut files = Vec::<DirEntry>::new();

    for path in dir {
        let path = path.unwrap();

        if path.metadata().unwrap().is_dir() {
            let mut f = read_dir(path.path().read_dir().unwrap());
            files.append(&mut f);
            continue;
        }
        files.push(path);
    }

    files
}
