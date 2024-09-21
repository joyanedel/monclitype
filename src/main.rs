use clap::Parser;
use sentences::pick_random_words_from_dictionary;
use std::{
    fs::read_to_string,
    io::{self},
};
use views::typing_playground::TypingPlayground;

mod sentences;
mod splitter;
mod tui;
mod types;
mod views;

const TARGET_SENTENCE_LENGTH: usize = 25;

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value = "./dictionaries/default.txt")]
    dictionary_dir: String,
}

fn main() -> io::Result<()> {
    // setup terminal
    let mut terminal = tui::init()?;
    let args = Args::parse();

    let dictionary = read_to_string(args.dictionary_dir).unwrap();
    let dictionary = dictionary.lines().collect();

    let target_word =
        pick_random_words_from_dictionary(&dictionary, TARGET_SENTENCE_LENGTH).join(" ");
    let app_result = TypingPlayground::default().run(&mut terminal, target_word);

    tui::restore()?;
    app_result
}
