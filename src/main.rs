mod logger;
mod mapper;
mod chunking;
mod math;

use logger::LogLevel;
use std::{fs::{self, ReadDir, DirEntry}, collections::HashMap};

fn main() {
    let args = &std::env::args().collect::<Vec<String>>();
    if args.len() == 1 {
        logger::log("Please provide a directory as program argument!".to_string(), LogLevel::Fatal);
        return;
    }

    let files = read_dir(fs::read_dir(&args[1]).unwrap());
    let chunks = chunking::get_chunks(files);

    let percentages = calc_percentages(&chunks);
    print_summary(&percentages);
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

fn calc_percentages(chunks: &HashMap<String, u64>) -> HashMap<String, f32> {
    let chunk_vec: Vec<u64> = chunks.values().cloned().collect();
    let chunk_sum = math::get_sum(&chunk_vec);

    let mut map = HashMap::<String, f32>::new();

    for lang in chunks {
        let percent = math::get_percent(&chunk_sum, lang.1);
        map.insert(lang.0.to_string(), percent * 100.0);
    }

    map
}

fn print_summary(data: &HashMap<String, f32>) {
    println!("\n====== SUMMARY ======");
    for info in data {
        println!("{}: {:.2}%", info.0, info.1);
    }
    println!("=====================");
} 
