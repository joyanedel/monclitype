use clap::Parser;
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use itertools::{EitherOrBoth, Itertools};
use ratatui::{
    layout::Alignment,
    style::Style,
    symbols::border,
    text::{Line, Span},
    widgets::{
        block::{Position, Title},
        Block, Borders, Paragraph, Widget, Wrap,
    },
    Frame,
};
use sentences::pick_random_words_from_dictionary;
use splitter::get_current_game_status;
use std::{
    fs::read_to_string,
    io::{self},
    time::Instant,
};

use types::{KeyEventSource, WordGameStatus, WordMatch};

mod sentences;
mod splitter;
mod tui;
mod types;

const TITLE_BLOCK: &str = "MoncliType";
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
    let app_result = App::default().run(&mut terminal, target_word);

    tui::restore()?;
    app_result
}

#[derive(Debug, Default)]
pub struct App {
    events: Vec<KeyEventSource>,
    target_word: String,
    exit: bool,
}

impl App {
    pub fn run(&mut self, terminal: &mut tui::Tui, target_word: String) -> io::Result<()> {
        self.target_word = target_word;
        loop {
            terminal.draw(|frame| self.render_frame(frame))?;
            self.handle_events()?;

            if self.exit {
                break;
            }
        }
        Ok(())
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

impl Widget for &App {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        let title = Title::from(TITLE_BLOCK);
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
        let already_written_words_span = build_word(&already_written_words_vec);
        let current_word_spans = build_word(&current_word);
        let future_words = match future_words {
            Some(v) => format!(" {}", v),
            None => String::new(),
        };
        let future_words_chars: Vec<EitherOrBoth<char, char>> = future_words
            .chars()
            .map(|c| EitherOrBoth::Right(c))
            .collect();
        let future_words_spans = build_word(&future_words_chars);

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

fn build_word(word: &WordMatch) -> Vec<Span> {
    word.iter().map(build_span_char).collect_vec()
}

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
        EitherOrBoth::Both(a, _) => a,
        EitherOrBoth::Left(v) => v,
        EitherOrBoth::Right(v) => v,
    };
    Span::styled(letter.to_string(), Style::default().fg(color))
}
