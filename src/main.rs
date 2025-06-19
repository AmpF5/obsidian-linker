use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use dotenv::dotenv;
use walkdir::{DirEntry, WalkDir};

// TODO: Add path as a second value
#[derive(Debug)]
struct Topics {
    h1: Vec<String>,
    h2: Vec<String>,
    h3: Vec<String>,
}

impl Topics {
    fn new(files: Vec<DirEntry>) -> Topics {
        let mut h1: Vec<String> = vec![];
        let mut h2: Vec<String> = vec![];
        let mut h3: Vec<String> = vec![];

        for f in files {
            match File::open(f.path()) {
                Ok(f) => {
                    let lines = BufReader::new(f).lines();

                    for line in lines.map_while(Result::ok) {
                        let sl: Vec<&str> = line.splitn(2, ' ').collect();
                        // TODO: Add header sanetization
                        match sl[0] {
                            "#" => {
                                h1.push(sl[1].to_string());
                            }
                            "##" => {
                                h2.push(sl[1].to_string());
                            }
                            "###" => {
                                h3.push(sl[1].to_string());
                            }
                            _ => {}
                        }
                    }
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
    dotenv().ok();

    let dir_path = std::env::var("DIR_PATH").expect("DIR_PATH is not set");
    let file_path = std::env::var("FILE_PATH").expect("FILE_PATH is not set");

    // TODO: remove reference to itself(file)
    let files_in_dir: Vec<DirEntry> = WalkDir::new(dir_path)
        .max_depth(3)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| {
            e.path().extension() == Some("md".as_ref())
                && e.path().to_str() != Some(file_path.as_str())
        })
        .collect();

    println!("=== FILES ===");

    for (i, path) in files_in_dir.iter().enumerate() {
        println!("{}. {}", i, path.file_name().display());
    }

    match File::open(file_path) {
        Ok(f) => {
            let lines = BufReader::new(f).lines();

            let topics = Topics::new(files_in_dir);

            println!("=== TOPICS ===");
            println!("{:?}", topics);
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
