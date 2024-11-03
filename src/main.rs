use clap::{ArgGroup, Parser};
use std::collections::HashMap;
use std::process::exit;
use zoega::*;

#[derive(Parser)]
#[command(name = "Geir Zoega dictionary searcher")]
#[command(about="A CLI to search through Geir Zoega dictionary of Old Norse Language", long_about=None)]
#[command(author = "merimacfairbairn")]
#[command(version = "v1.7.0")]
#[command(group(
        ArgGroup::new("mode")
        .required(true)
        .args(&[
            "word",
            "search",
            "history",
            "clear",
            "favorite",
            "unfavorite",
            "show_favorites",
            "word_of_the_day",
            "random",
        ])
))]
#[command(group(
        ArgGroup::new("display")
        .args(&["limit", "all"])
))]
#[command(group(
        ArgGroup::new("search_type")
        .args(&["search", "fuzzy"])
))]
struct Cli {
    /// The input word to search for exact match
    #[arg(group = "mode")]
    word: Option<String>,

    /// Custom regex pattern for search
    #[arg(short = 's', long, group = "mode", value_name = "PATTERN")]
    search: Option<String>,

    /// Number of suggestions to display
    #[arg(
        short = 'n',
        long,
        default_value_t = 5,
        group = "display",
        value_name = "NUM"
    )]
    limit: usize,

    /// Display all matches ignoring the limit
    #[arg(short, long, group = "display")]
    all: bool,

    /// Show search history
    #[arg(long, group = "mode")]
    history: bool,

    /// Clear search history
    #[arg(long, group = "mode")]
    clear: bool,

    /// Add a word to favorites
    #[arg(long, group = "mode", value_name = "WORD")]
    favorite: Option<String>,

    /// Remove a word from favorites
    #[arg(long, group = "mode", value_name = "WORD")]
    unfavorite: Option<String>,

    /// Display favorites
    #[arg(long, group = "mode")]
    show_favorites: bool,

    /// Display word of the day
    #[arg(long, group = "mode")]
    word_of_the_day: bool,

    /// Display a random word
    #[arg(long, short, group = "mode")]
    random: bool,

    /// Enable fuzzy search
    #[arg(long, short, group = "search_type")]
    fuzzy: bool,

    /// Change fuzzy search level
    #[arg(
        long,
        short = 'l',
        default_value_t = 2,
        value_name = "NUM",
        requires("fuzzy")
    )]
    fuzzy_level: usize,
}

fn main() {
    let args = Cli::parse();

    let word = args.word.as_deref();
    let word_to_definitions = get_default();

    if args.random {
        match random::get_random_word(&word_to_definitions) {
            Some(word) => print_definitions(&word, &word_to_definitions),
            None => println!("No words found in dictionary"),
        }
        exit(0);
    }

    if args.word_of_the_day {
        let word_of_the_day = random::get_word_of_the_day(&word_to_definitions);
        print_definitions(&word_of_the_day, &word_to_definitions);
        exit(0);
    }

    if let Some(word) = args.favorite {
        favorites::add(&word);
        exit(0);
    } else if let Some(word) = args.unfavorite {
        favorites::remove(&word);
        exit(0);
    } else if args.show_favorites {
        let favorites = favorites::get();
        if favorites.is_empty() {
            println!("No favorites yet");
            exit(0);
        }
        for word in favorites {
            println!(" - {}", word);
        }
        exit(0);
    }

    if args.history {
        history::display();
        exit(0);
    } else if args.clear {
        history::clear();
        exit(0);
    }

    if let Some(word) = word {
        history::add(word);
        print_definitions(&word, &word_to_definitions);
    } else if let Some(pattern) = &args.search {
        history::add(format!("\"{pattern}\"").as_str());
    }

    let suggestions = if args.fuzzy {
        fuzzy_search(
            word,
            word_to_definitions,
            args.fuzzy_level,
            args.limit,
            args.all,
        )
    } else {
        suggest_words(
            word,
            &word_to_definitions,
            args.search.as_deref(),
            args.limit,
            args.all,
        )
    };

    if suggestions.is_empty() || word.unwrap_or("---").len() <= 2 {
        println!("No matches found");
    } else {
        println!("Did you mean one of these?");
        for suggestion in suggestions {
            println!(" - {}", suggestion);
        }
    }
}

fn print_definitions(word: &str, data: &HashMap<String, Vec<String>>) {
    if let Some(definitions) = data.get(word) {
        println!("Definitions for {}:", word);
        definitions
            .iter()
            .for_each(|definition| println!("{}", definition));
        exit(0);
    }

    println!("No definitions found for '{}'", word);
}
