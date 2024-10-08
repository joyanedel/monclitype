use clap::Parser;
use sentences::pick_random_words_from_dictionary;
use std::{
    fs::read_to_string,
    io::{self},
};
use views::{run::Runnable, statistics::StatisticsView, typing_playground::TypingPlayground};

mod sentences;
mod splitter;
mod tui;
mod types;
mod views;

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value = "./dictionaries/default.txt")]
    dictionary_dir: String,
    #[arg(short, long, default_value = "25")]
    total_words: usize,
}

fn main() -> io::Result<()> {
    // setup terminal
    let args = Args::parse();
    let mut terminal = tui::init()?;

    let dictionary = read_to_string(args.dictionary_dir).unwrap();
    let dictionary = dictionary.lines().collect();

    let target_word = pick_random_words_from_dictionary(&dictionary, args.total_words).join(" ");

    // Typing playground
    let mut typing_playground = TypingPlayground::new(target_word.clone());

    typing_playground
        .run(&mut terminal)
        .expect("There was something wrong");

    // Statistics view
    StatisticsView::new(typing_playground.get_user_events(), target_word)
        .run(&mut terminal)
        .expect("Something went wrong with statistics");

    tui::restore()?;
    Ok(())
}
