use clap::{ArgGroup, Parser};
use multimap::MultiMap;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::{fs, process::exit};
use std::path::Path;
use std::io::{self, BufRead, Write};

const DICTIONARY_FILE: &str = "data/dictionary.json";
const HISTORY_FILE: &str = "data/history.txt";
const HISTORY_LIMIT: usize = 70;

#[derive(Serialize, Deserialize, Debug)]
struct Pair {
    word: String,
    definitions: Vec<String>,
}

#[derive(Parser)]
#[command(name = "Cleasby-Vigfusson dictionary searcher")]
#[command(about="A CLI to search through Cleasby-Vigfusson dictionary of Old Norse Language", long_about=None)]
#[command(author = "merimacfairbairn")]
#[command(version = "v1.3.1")]
#[command(group(
        ArgGroup::new("mode")
        .required(true)
        .args(&["word", "search", "history", "clear_history"])
))]
#[command(group(
        ArgGroup::new("display")
        .args(&["limit", "all"])
))]
struct Cli {
    /// The input word to search for exact match
    #[arg(group = "mode")]
    word: Option<String>,

    /// Custom regex pattern for search
    #[arg(short = 's', long, group = "mode", value_name = "PATTERN")]
    search: Option<String>,

    /// Number of suggestions to display
    #[arg(short = 'n', long, default_value_t = 5, group = "display")]
    limit: usize,

    /// Display all matches ignoring the limit
    #[arg(short, long, group = "display")]
    all: bool,

    /// Show search history
    #[arg(long, group = "mode")]
    history: bool,

    /// Clear search history
    #[arg(long, group = "mode")]
    clear_history: bool,
}

fn main() {
    let args = Cli::parse();

    let word = args.word.as_deref();
    let pairs_map = get_pairs_map();

    if args.history {
        display_history();
        exit(0);
    } else if args.clear_history {
        clear_history();
        exit(0);
    }

    if let Some(word) = word {
        add_to_history(word);
        if let Some(definitions) = pairs_map.get(word) {
            println!("Definitons for: '{}':", word);
            definitions
                .iter()
                .for_each(|definition| println!("{}\n", definition));
            suggest_upper_if_exists(&word, &pairs_map);
            exit(1);
        }
    } else if let Some(pattern) = &args.search {
        add_to_history(pattern);
    }

    let suggestions = suggest_words(
        word,
        &pairs_map,
        args.search.as_deref(),
        args.limit,
        args.all,
    );

    if suggestions.is_empty() || word.unwrap_or("---").len() <= 2 {
        println!("No matches found");
    } else {
        println!("Did you mean one of these?");
        for suggestion in suggestions {
            println!(" - {}", suggestion);
        }
    }
}

fn suggest_words(
    word: Option<&str>,
    data: &MultiMap<String, Vec<String>>,
    pattern: Option<&str>,
    limit: usize,
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
        matches.into_iter().map(|(key, _)| key).collect()
    } else {
        matches
            .into_iter()
            .take(limit)
            .map(|(key, _)| key)
            .collect()
    }
}

fn suggest_upper_if_exists(word: &str, pairs_map: &MultiMap<String, Vec<String>>) {
    let word_upper = &word.to_uppercase();
    if pairs_map.get(word_upper).is_some() && word.chars().all(|c| c.is_lowercase()) {
        println!("See capitalized version: {}", word_upper);
    }
}

fn get_pairs_map() -> MultiMap<String, Vec<String>> {
    let contents = fs::read_to_string(DICTIONARY_FILE).expect("Could not load dictionary");
    let json: Vec<Pair> = serde_json::from_str(&contents).expect("Malformed JSON");

    let mut pairs_map = MultiMap::new();
    for pair in &json {
        pairs_map.insert(pair.word.clone(), pair.definitions.clone());
    }

    pairs_map
}

fn add_to_history(term: &str) {
    let mut history = if Path::new(HISTORY_FILE).exists() {
        let file = fs::File::open(HISTORY_FILE).expect("Failed to open history file");
        io::BufReader::new(file)
            .lines()
            .filter_map(Result::ok)
            .collect::<Vec<String>>()
    } else {
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

fn display_history() {
    if Path::new(HISTORY_FILE).exists() {
        let history = fs::read_to_string(HISTORY_FILE).expect("Failed to read history file");
        println!("Search History:\n{}", history);
    } else {
        println!("No search history found");
    }
}

fn clear_history() {
    if Path::new(HISTORY_FILE).exists() {
        fs::remove_file(HISTORY_FILE).expect("Failed to clear history");
        println!("Search history cleared");
    } else {
        println!("No history to clear");
    }
}
