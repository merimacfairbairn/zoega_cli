# Zoega CLI

[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

A command-line interface application designed for searching words through "Concise Dictionary of Old Icelandic" by Geir Zoega.

## About the Dictionary

"A Concise Dictionary of Old Icelandic" dictionary was published in 1910 by Geir Zoëga, which leads to there being many public domain versions of the book available.
Zoëga's attempt was to made easier-to-approach version of the more full Cleasby-Vigfusson dictionary, specifically for beginners and those interested in Old Icelandic prose writing.
The dictionary consists of 29 000+ Old Icelandic words with English translations.

---

## Table of Contents

- [Features](#features)
- [Installation](#installation)
- [Usage](#usage)
- [License](#license)
- [Contact](#contact)
- [Credits](#credits)

---

## Features

- **Search Old Norse Dictionary**: Efficiently search through a comprehensive Old Norse dictionary directly from the command line.
- **Fuzzy Search**: Utilize fuzzy search capabilities to find words even with approximate spellings.
- **Save Favorites**: Save frequently accessed words to a favorites list for quick reference.
- **History**: The app saves up to 70 words in the search history.
- **Word of the Day**: Receive a daily Old Norse word to enhance vocabulary learning.
- **Random Word Feature**: Discover new words by generating random entries from the dictionary.

---

## Installation

This section is under development

### Prerequisites
- Install [Rust](https://www.rust-lang.org/tools/install) (version 1.60 or higher) using `rustup`.

### Using Cargo Install
If you have Rust and Cargo (Rust's package manager) installed, you can install the application directly from the source repository:
```
cargo install --git https://github.com/merimacfairbairn/zoega_cli.git
```

### From Source
1. Clone the repository:
```
git clone https://github.com/merimacfairbairn/zoega_cli.git
cd zoega_cli
```

2. Build the application:
```
cargo build --release
```

3. After the build completes, the binary will be located in the `target/release` directory. You can copy it to a directory included in your system's `PATH`, or run it directly:
```
./target/release/zoega_cli
```

---

## Usage

To get started, run:
```
zoega --help
```

### Common Commands
- **word**: `zoega_cli word [OPTIONS] <WORD|--search <PATTERN>>`
  Searches for a word or a pattern in the dictionary.
  In case of an exact match, displays the definitions. Otherwise suggests words beginning with the query.
  If -p flag is set, suggests words that match the given regex pattern.
  If -f flag is set, suggests words based on levinshtein distance.

- **hist**: `zoega_cli hist [OPTIONS]`
  Prints the history by default(max 70 recent searches).
  Using -c flag clears the history.

- **fav**: `zoega_cli fav [OPTIONS]`
  Prints the favorites by default.
  Using -a flag adds a word to favorites.
  Using -r flag removes a word from favorites.

---

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

---

## Contact

If you have any questions or feedback, feel free to contact:
- **Author Name**: Merimacfairbairn
- **Email**: [merimacfairbairn.git@gmail.com](mailto:merimacfairbairn.git@gmail.com)

---

## Credits

Additional credits to [stscoundrel](https://github.com/stscoundrel/old-icelandic-dictionary-py) for markup and no-markup JSON versions of Zoega's dictionary.

