use std::{fs::{self, ReadDir, DirEntry}};

fn main() {
    let target_dir = &std::env::args().collect::<Vec<String>>()[1];
    let path = fs::read_dir(target_dir).unwrap();

    for p in read_dir(path) {
        println!("{}", p.path().display());
    }
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
