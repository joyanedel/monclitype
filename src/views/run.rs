use std::io;

use crate::tui::Tui;

pub trait Runnable {
    /// Method to run this view in the provided terminal
    fn run(&mut self, terminal: &mut Tui) -> io::Result<()>;
}
