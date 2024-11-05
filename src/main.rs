use clap::{ArgGroup, Args, Parser, Subcommand};
use std::collections::HashMap;
use std::process::exit;
use zoega::*;

#[derive(Parser)]
#[command(name = "Geir Zoega dictionary searcher")]
#[command(about="A CLI to search through Geir Zoega concise dictionary of Old Icelandic Language", long_about=None)]
#[command(author = "merimacfairbairn")]
#[command(version = "v1.8.0")]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Search words
    Word(WordArgs),

    /// Manage history
    Hist(HistArgs),

    /// Manage favorites
    Fav(FavArgs),

    /// Random word definitions
    Rand(RandArgs),
}

#[derive(Args)]
#[command(group(
        ArgGroup::new("mode")
        .required(true)
        .args(&["word", "search"])
))]
#[command(group(
        ArgGroup::new("display")
        .args(&["all", "limit"])
))]
#[command(group(
        ArgGroup::new("search_type")
        .args(&["fuzzy", "search"])
))]
struct WordArgs {
    /// The input word to search for exact match
    #[arg(group = "mode")]
    word: Option<String>,

    /// Custom regex pattern for search
    #[arg(short = 's', long, group = "mode", value_name = "PATTERN")]
    search: Option<String>,

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
}

#[derive(Args)]
struct HistArgs {
    /// Clear history
    #[arg(long, short = 'c')]
    clear_history: bool,
}

#[derive(Args)]
#[command(group(
        ArgGroup::new("fav_options")
        .args(&["add", "remove"])
))]
struct FavArgs {
    /// Add to favorites
    #[arg(long, short = 'a', value_name = "WORD")]
    add: Option<String>,

    /// Remove from favorites
    #[arg(long, short = 'r', value_name = "WORD")]
    remove: Option<String>,
}

#[derive(Args)]
struct RandArgs {
    /// Display word of the day
    #[arg(long, short = 'd', group = "mode")]
    word_of_the_day: bool,
}

fn main() {
    let args = Cli::parse();
    match &args.command {
        Commands::Word(word_args) => {
            let word = word_args.word.as_deref();
            let word_to_definitions = get_default();

            if let Some(word) = word {
                history::add(word);
                print_definitions(&word, &word_to_definitions);
            } else if let Some(pattern) = &word_args.search {
                history::add(format!("\"{pattern}\"").as_str());
            }

            let suggestions = if !word_args.fuzzy {
                suggest_words(
                    word,
                    &word_to_definitions,
                    word_args.search.as_deref(),
                    word_args.limit,
                    word_args.all,
                )
            } else {
                fuzzy_suggest(
                    word,
                    word_to_definitions,
                    word_args.fuzzy_level,
                    word_args.limit,
                    word_args.all,
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

        Commands::Hist(hist_args) => {
            if hist_args.clear_history {
                history::clear();
                exit(0);
            }

            history::display();
        }

        Commands::Fav(fav_args) => {
            if let Some(word) = &fav_args.add {
                favorites::add(&word);
                exit(0);
            } else if let Some(word) = &fav_args.remove {
                favorites::remove(&word);
                exit(0);
            }

            let favorites = favorites::get();
            if favorites.is_empty() {
                println!("No favorites yet");
                exit(0);
            }

            for word in favorites {
                println!(" - {}", word);
            }
        }

        Commands::Rand(args) => {
            let word_to_definitions = get_default();

            if args.word_of_the_day {
                let word_of_the_day = random::get_word_of_the_day(&word_to_definitions);
                print_definitions(&word_of_the_day, &word_to_definitions);
                exit(0);
            }

            match random::get_random_word(&word_to_definitions) {
                Some(word) => print_definitions(&word, &word_to_definitions),
                None => println!("No words found in dictionary"),
            }
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
