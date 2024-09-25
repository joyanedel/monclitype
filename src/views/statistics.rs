use std::io;

use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout},
    style::Color,
    widgets::{
        block::{Position, Title},
        Block, Paragraph, Widget,
    },
    Frame,
};

use crate::{
    sentences::{build_sentence, verify_sentence_input},
    tui,
    types::KeyEventSource,
};

use super::run::Runnable;

#[derive(Debug, Default)]
pub struct StatisticsView {
    target_word: String,
    user_events: Vec<KeyEventSource>,
    exit: bool,
}

impl StatisticsView {
    pub fn new(user_events: Vec<KeyEventSource>, target_word: String) -> Self {
        Self {
            user_events,
            target_word,
            exit: false,
        }
    }

    fn render_frame(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.size());
    }

    fn handle_events(&mut self) -> Option<()> {
        match event::read().ok()? {
            Event::Key(key_event)
                if key_event.kind == KeyEventKind::Press && key_event.code == KeyCode::Esc =>
            {
                self.exit = true;
            }
            _ => {}
        };

        Some(())
    }
}

impl Runnable for StatisticsView {
    fn run(&mut self, terminal: &mut tui::Tui) -> io::Result<()> {
        loop {
            terminal.draw(|frame| self.render_frame(frame))?;
            self.handle_events();

            if self.exit {
                break;
            }
        }
        Ok(())
    }
}

impl Widget for &StatisticsView {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        let title = Title::from("Last Run");
        let instructions = Title::from("Press <ESC> to exit");
        let vertical_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![Constraint::Length(3), Constraint::Percentage(100)])
            .split(area);

        // render header
        let game_status =
            verify_sentence_input(&build_sentence(&self.user_events), &self.target_word);
        Block::bordered()
            .title(title.alignment(Alignment::Center))
            .render(vertical_layout[0], buf);
        let header_layout = Layout::horizontal(vec![Constraint::Min(0), Constraint::Min(0)])
            .vertical_margin(1)
            .horizontal_margin(1)
            .split(vertical_layout[0]);
        Paragraph::new(format!("Total pressed keys: {}", self.user_events.len()))
            .alignment(Alignment::Left)
            .render(header_layout[0], buf);
        Paragraph::new(format!(
            "Game status: {}",
            if game_status { "succeed" } else { "failure" }
        ))
        .style(if game_status {
            Color::Green
        } else {
            Color::Red
        })
        .alignment(Alignment::Right)
        .render(header_layout[1], buf);

        // render body
        let body = Block::bordered().title(
            instructions
                .alignment(Alignment::Center)
                .position(Position::Bottom),
        );

        body.render(vertical_layout[1], buf);
        // block.render(area, buf);
    }
}
