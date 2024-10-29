use clap::{ArgGroup, Parser};
use serde::{Deserialize, Serialize};

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
pub struct Cli {
    /// The input word to search for exact match
    #[arg(group = "mode")]
    pub word: Option<String>,

    /// Custom regex pattern for search
    #[arg(short = 's', long, group = "mode", value_name = "PATTERN")]
    pub search: Option<String>,

    /// Number of suggestions to display
    #[arg(short = 'n', long, default_value_t = 5, group = "display")]
    pub limit: usize,

    /// Display all matches ignoring the limit
    #[arg(short, long, group = "display")]
    pub all: bool,

    /// Show search history
    #[arg(long, group = "mode")]
    pub history: bool,

    /// Clear search history
    #[arg(long, group = "mode")]
    pub clear_history: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Pair {
    pub word: String,
    pub definitions: Vec<String>,
}
