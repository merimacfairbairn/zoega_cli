use crate::HashMap;
use chrono::prelude::*;
use rand::seq::IteratorRandom;
use std::fs;
use std::io::Write;

const WORD_OF_THE_DAY_FILE: &str = "data/word_of_the_day.txt";

pub fn get_word_of_the_day(data: &HashMap<String, Vec<String>>) -> String {
    let today = Local::now().format("%Y-%m-%d").to_string();

    if let Ok(file_content) = fs::read_to_string(WORD_OF_THE_DAY_FILE) {
        let mut lines = file_content.lines();
        if let (Some(saved_date), Some(saved_word)) = (lines.next(), lines.next()) {
            if saved_date == today {
                return saved_word.to_string();
            }
        }
    }

    calculate_new_word_of_the_day(data, &today)
}

fn calculate_new_word_of_the_day(data: &HashMap<String, Vec<String>>, today: &str) -> String {
    let word_index = Local::now().ordinal() as usize % data.len();
    let word_of_the_day = data.keys().nth(word_index).expect("No words found");

    let mut file = fs::File::create(WORD_OF_THE_DAY_FILE)
        .expect("Failed to open word of the day file for writing");
    writeln!(file, "{}", today).expect("Failed to write date");
    writeln!(file, "{}", word_of_the_day).expect("Failed to write word");

    return word_of_the_day.to_string();
}

pub fn get_random_word(data: &HashMap<String, Vec<String>>) -> Option<String> {
    let mut rng = rand::thread_rng();
    if let Some((random_word, _definitions)) = data.iter().choose(&mut rng) {
        return Some(random_word.to_string());
    }

    None
}
