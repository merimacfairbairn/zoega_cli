use std::collections::HashMap;
use std::fs;
use std::io::{BufRead, BufReader, Write};
use std::path::Path;

const FAVORITES_FILE: &str = "data/favorites.txt";

pub fn add(word: &str, data: &HashMap<String, Vec<String>>) {
    let mut favorites = get();

    if favorites.contains(&word.to_string()) {
        println!("'{}' is already in favorites", word);
        return;
    } else if !data.contains_key(word) {
        println!("'{}' is not in the dictionary", word);
        return;
    }

    favorites.push(word.to_string());
    save_favorites(&favorites);
    println!("'{}' has been added to favorites", word);
}

pub fn remove(word: &str) {
    let mut favorites = get();

    if !favorites.contains(&word.to_string()) {
        println!("'{}' is not in favorites", word);
        return;
    }

    favorites.retain(|w| w != word);
    save_favorites(&favorites);
    println!("'{}' has been removed from favorites", word);
}

pub fn get() -> Vec<String> {
    if Path::new(FAVORITES_FILE).exists() {
        let file = fs::File::open(FAVORITES_FILE).expect("Failed to open favorites file");
        BufReader::new(file)
            .lines()
            .map_while(Result::ok)
            .collect::<Vec<String>>()
    } else {
        fs::create_dir_all("data").expect("Failed to create 'data' directory");
        vec![]
    }
}

fn save_favorites(favorites: &[String]) {
    let mut file = fs::File::create(FAVORITES_FILE).expect("Failed to open favorites file");
    for word in favorites {
        writeln!(file, "{}", word).expect("Failed to write to favorites file");
    }
}
