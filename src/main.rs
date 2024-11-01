use clap::{ArgGroup, Parser};
use cleasby::*;
use std::process::exit;

mod history;

#[derive(Parser)]
#[command(name = "Cleasby-Vigfusson dictionary searcher")]
#[command(about="A CLI to search through Cleasby-Vigfusson dictionary of Old Norse Language", long_about=None)]
#[command(author = "merimacfairbairn")]
#[command(version = "v1.3.3")]
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
    let word_to_definitions = get_word_to_definitons_map();

    if args.history {
        history::display_history();
        exit(0);
    } else if args.clear_history {
        history::clear_history();
        exit(0);
    }

    if let Some(word) = word {
        history::add_to_history(word);
        if let Some(definitions) = word_to_definitions.get(word) {
            println!("Definitons for: '{}':", word);
            definitions
                .iter()
                .for_each(|definition| println!("{}\n", definition));
            if is_upper_in_map(&word, &word_to_definitions) {
                println!("See the capitalized version: {}", &word.to_uppercase());
            }
            exit(0);
        }
    } else if let Some(pattern) = &args.search {
        history::add_to_history(format!("\"{pattern}\"").as_str());
    }

    let suggestions = suggest_words(
        word,
        &word_to_definitions,
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
