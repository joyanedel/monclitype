use crossterm::event::KeyCode;
use itertools::EitherOrBoth;
use std::time::Instant;

#[derive(Debug)]
pub struct KeyEventSource {
    pub key: KeyCode,
    pub timestamp: Instant,
}

#[derive(PartialEq, Debug)]
pub struct InputTargetChar(pub Option<char>, pub Option<char>);

pub type WordMatch = Vec<EitherOrBoth<char>>;

#[derive(PartialEq, Debug)]
pub struct WordGameStatus(pub Vec<WordMatch>, pub WordMatch, pub Option<String>);

#[derive(Debug)]
pub struct GameFinished;
