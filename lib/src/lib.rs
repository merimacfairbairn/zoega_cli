use regex::{self, Regex};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use strsim::levenshtein;

pub mod favorites;
pub mod history;
pub mod random;

#[derive(Serialize, Deserialize, Debug)]
struct Pair {
    word: String,
    definitions: Vec<String>,
}

pub fn get_default() -> HashMap<String, Vec<String>> {
    get_word_to_definitons_map(true)
}

pub fn get_markup() -> HashMap<String, Vec<String>> {
    get_word_to_definitons_map(false)
}

fn get_word_to_definitons_map(default: bool) -> HashMap<String, Vec<String>> {
    let contents = if default {
        include_str!("../data/default.json")
    } else {
        include_str!("../data/markup.json")
    };

    let json: Vec<Pair> = serde_json::from_str(contents).expect("Malformed JSON");

    let mut word_to_definitions = HashMap::new();
    for pair in json {
        word_to_definitions.insert(pair.word.clone(), pair.definitions.clone());
    }

    word_to_definitions
}

pub fn suggest_words(
    word: Option<&str>,
    data: &HashMap<String, Vec<String>>,
    pattern: Option<&str>,
    limit: usize,
    offset: usize,
    display_all: bool,
) -> Vec<String> {
    let regex_pattern = match pattern {
        Some(pat) => pat.to_string(),
        None => {
            let word = word.expect("Word must be provided if the pattern is not specified");
            format!(r"(?i)^{}.*", regex::escape(word))
        }
    };

    let regex = Regex::new(&regex_pattern).expect("Invalid regex pattern");

    let mut matches: Vec<(String, usize)> = data
        .keys()
        .filter(|key| regex.is_match(key))
        .map(|key| {
            let extra_chars = key.len().saturating_sub(word.unwrap_or("").len());
            (key.clone(), extra_chars)
        })
        .collect();

    matches.sort_by_key(|&(_, extra_chars)| extra_chars);

    if display_all {
        return matches.into_iter().map(|(key, _)| key).collect();
    }

    matches
        .into_iter()
        .skip(offset)
        .take(limit)
        .map(|(key, _)| key)
        .collect()
}

pub fn fuzzy_suggest(
    word: Option<&str>,
    data: &HashMap<String, Vec<String>>,
    fuzzy_level: usize,
    limit: usize,
    offset: usize,
    display_all: bool,
) -> Vec<String> {
    let word = word.expect("Word must be provided if the pattern is not specified");
    let mut matches: Vec<(String, usize)> = data
        .keys()
        .map(|key| (key.clone(), levenshtein(word, key)))
        .filter(|&(_, dist)| dist <= fuzzy_level)
        .collect();

    matches.sort_by(|a, b| {
        a.1.cmp(&b.1).then_with(|| a.0.cmp(&b.0))
    });

    if display_all {
        return matches.into_iter().map(|(key, _)| key).collect();
    }

    matches
        .into_iter()
        .skip(offset)
        .take(limit)
        .map(|(key, _)| key)
        .collect()
}
