use std::io;

use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::{
    layout::Alignment,
    widgets::{
        block::{Position, Title},
        Block, Paragraph, Widget,
    },
    Frame,
};

use crate::{tui, types::KeyEventSource};

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

        let block = Block::default()
            .title(title.alignment(Alignment::Center))
            .title(
                instructions
                    .alignment(Alignment::Center)
                    .position(Position::Bottom),
            );

        block.render(area, buf);
    }
}
