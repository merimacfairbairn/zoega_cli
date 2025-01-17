use std::fs;
use std::io::{self, BufRead, Write};
use std::path::Path;

static HISTORY_FILE: &str = "data/history.txt";
const HISTORY_LIMIT: usize = 70;

pub fn add(term: &str) {
    let mut history = if Path::new(HISTORY_FILE).exists() {
        let file = fs::File::open(HISTORY_FILE).expect("Failed to open history file");
        io::BufReader::new(file)
            .lines()
            .map_while(Result::ok)
            .collect::<Vec<String>>()
    } else {
        fs::create_dir_all("data").expect("Failed to create 'data' directory");
        vec![]
    };

    if history.contains(&term.to_string()) {
        return;
    }

    history.push(term.to_string());

    if history.len() > HISTORY_LIMIT {
        history.drain(0..(history.len() - HISTORY_LIMIT));
    }

    let mut file = fs::File::create(HISTORY_FILE).expect("Failed to open history file for writing");
    for entry in history {
        writeln!(file, "{}", entry).expect("Failed to write to history file");
    }
}

pub fn display() {
    if Path::new(HISTORY_FILE).exists() {
        let history = fs::read_to_string(HISTORY_FILE).expect("Failed to read history file");
        println!("Search History:\n{}", history);
    } else {
        println!("No search history found");
    }
}

pub fn clear() {
    if Path::new(HISTORY_FILE).exists() {
        fs::remove_file(HISTORY_FILE).expect("Failed to clear history");
        println!("Search history cleared");
    } else {
        println!("No history to clear");
    }
}
