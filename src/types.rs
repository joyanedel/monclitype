use crossterm::event::KeyCode;
use std::time::Instant;

#[derive(Debug)]
pub struct KeyEventSource {
    pub key: KeyCode,
    pub timestamp: Instant,
}
