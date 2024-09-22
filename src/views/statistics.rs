use ratatui::widgets::Widget;

use crate::types::KeyEventSource;

use super::run::Runnable;

#[derive(Debug)]
pub struct StatisticsView {
    target_word: String,
    user_events: Vec<KeyEventSource>,
}

impl StatisticsView {
    pub fn new(user_events: Vec<KeyEventSource>, target_word: String) -> Self {
        Self {
            user_events,
            target_word,
        }
    }
}

impl Runnable for StatisticsView {
    fn run(&mut self, terminal: &mut crate::tui::Tui) -> std::io::Result<()> {
        Ok(())
    }
}

impl Widget for &StatisticsView {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
    }
}
