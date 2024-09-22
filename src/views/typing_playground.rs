use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use itertools::{EitherOrBoth, Itertools};
use ratatui::layout::Alignment;
use ratatui::style::Style;
use ratatui::symbols::border;
use ratatui::text::{Line, Span};
use ratatui::widgets::block::{Position, Title};
use ratatui::widgets::{Block, Borders, Paragraph, Widget, Wrap};
use ratatui::Frame;

use crate::splitter::get_current_game_status;
use crate::tui;
use crate::types::{KeyEventSource, WordGameStatus, WordMatch};
use std::io;
use std::time::Instant;

use super::run::Runnable;

#[derive(Debug, Default)]
pub struct TypingPlayground {
    events: Vec<KeyEventSource>,
    target_word: String,
    exit: bool,
}

impl TypingPlayground {
    pub fn new(target_word: String) -> Self {
        let mut _self = TypingPlayground::default();
        _self.target_word = target_word;

        _self
    }

    pub fn get_user_events(&self) -> Vec<KeyEventSource> {
        self.events[..].to_vec()
    }

    fn render_frame(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.size())
    }

    fn handle_events(&mut self) -> io::Result<()> {
        match event::read()? {
            Event::Key(key_event)
                if key_event.kind == KeyEventKind::Press && key_event.code == KeyCode::Esc =>
            {
                self.exit = true;
            }
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.events.push(KeyEventSource {
                    key: key_event.code,
                    timestamp: Instant::now(),
                });
            }
            _ => {}
        };

        Ok(())
    }
}

impl Runnable for TypingPlayground {
    fn run(&mut self, terminal: &mut tui::Tui) -> io::Result<()> {
        loop {
            terminal.draw(|frame| self.render_frame(frame))?;
            self.handle_events()?;

            if self.exit {
                break;
            }

            // check if user has typed last word
            let game_status = get_current_game_status(&self.events, &self.target_word);
            if game_status.is_err() {
                break;
            }
            let WordGameStatus(_, current_word, future_words) = game_status.unwrap();
            if future_words.is_some_and(|word| word.len() > 0) {
                continue;
            }

            // there is no remaining letters in target word to fill
            if current_word
                .iter()
                .take_while(|w| w.is_both())
                .collect::<Vec<_>>()
                .len()
                == current_word.len()
            {
                break;
            }
        }
        Ok(())
    }
}

impl Widget for &TypingPlayground {
    /// Render the TypingPlayground widget
    ///
    /// The widget will render the current state of the game
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        let title = Title::from("MoncliType");
        let instructions = Title::from("Press <ESC> to exit");

        let block = Block::default()
            .title(title.alignment(Alignment::Left))
            .title(
                instructions
                    .alignment(Alignment::Center)
                    .position(Position::Bottom),
            )
            .borders(Borders::ALL)
            .border_set(border::THICK);

        let game_status = get_current_game_status(&self.events, &self.target_word);
        if game_status.is_err() {
            return;
        }

        let WordGameStatus(already_written_words, current_word, future_words) =
            game_status.unwrap();

        let already_written_words_vec = already_written_words
            .iter()
            .flat_map(|inner| {
                inner
                    .iter()
                    .chain(std::iter::once(&EitherOrBoth::Both(' ', ' ')))
            })
            .cloned()
            .collect_vec();
        let already_written_words_span = build_word_span(&already_written_words_vec);
        let current_word_spans = build_word_span(&current_word);
        let future_words = match future_words {
            Some(v) => format!(" {}", v),
            None => String::new(),
        };
        let future_words_chars: Vec<EitherOrBoth<char, char>> = future_words
            .chars()
            .map(|c| EitherOrBoth::Right(c))
            .collect();
        let future_words_spans = build_word_span(&future_words_chars);

        let all_words = already_written_words_span
            .iter()
            .chain(current_word_spans.iter())
            .chain(future_words_spans.iter())
            .cloned()
            .collect_vec();

        let line = Line::from(all_words);

        Paragraph::new(line)
            .left_aligned()
            .block(block)
            .style(
                Style::new()
                    .bg(ratatui::style::Color::Rgb(10, 10, 10))
                    .fg(ratatui::style::Color::Yellow),
            )
            .wrap(Wrap { trim: true })
            .render(area, buf);
    }
}

/// Build a span for a word. The result is a vector of spans, one for each character
fn build_word_span(word: &WordMatch) -> Vec<Span> {
    word.iter().map(build_span_char).collect_vec()
}

/// Build a span for a character
/// If the character is the same in both sides, it will be white
///
/// If the character is different, it will be red and the target character will be rendered
///
/// If the character is only in the target word, it will be grey
///
/// If the character is only in the user input, it will be red
fn build_span_char(pair_of_chars: &EitherOrBoth<char>) -> Span {
    let color = match pair_of_chars {
        EitherOrBoth::Left(_) => ratatui::style::Color::Red,
        EitherOrBoth::Right(_) => ratatui::style::Color::Rgb(50, 50, 50),
        EitherOrBoth::Both(a, b) => {
            if a == b {
                ratatui::style::Color::White
            } else {
                ratatui::style::Color::Red
            }
        }
    };
    let letter = match pair_of_chars {
        EitherOrBoth::Both(_, b) => b,
        EitherOrBoth::Left(v) => v,
        EitherOrBoth::Right(v) => v,
    };
    Span::styled(letter.to_string(), Style::default().fg(color))
}
