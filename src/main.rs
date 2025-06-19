use std::{
    fs::File,
    io::{BufRead, BufReader, Read},
};

use walkdir::{DirEntry, WalkDir};

#[derive(Debug)]
struct Topics {
    h1: Vec<String>,
    h2: Vec<String>,
    h3: Vec<String>,
}

impl Topics {
    fn new(files: Vec<DirEntry>) -> Topics {
        let h1: Vec<String> = vec![];
        let h2: Vec<String> = vec![];
        let h3: Vec<String> = vec![];

        for f in files {
            match File::open(f.path()) {
                Ok(f) => {
                    let lines = BufReader::new(f).lines();

                    for line in lines.map_while(Result::ok) {}
                }
                Err(e) => {
                    eprintln!("Couldn't open file {e}")
                }
            }
        }

        Topics { h1, h2, h3 }
    }
}

fn main() {
    let files_in_dir: Vec<DirEntry> = WalkDir::new(dir_path)
        .max_depth(3)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| e.path().extension() == Some("md".as_ref()))
        .collect();

    println!("=== FILES ===");
    for (i, path) in files_in_dir.iter().enumerate() {
        println!("{}. {}", i, path.file_name().display());
    }

    match File::open(file_path) {
        Ok(f) => {
            let lines = BufReader::new(f).lines();

            let topics = Topics::new(files_in_dir);

            println!("=== FILE CONTENT ===");
            for line in lines.map_while(Result::ok) {
                println!("{}", line);
            }
        }
        Err(e) => {
            eprintln!("Couldn't open file {e}");
        }
    }
}
