use std::io;

use ratatui::DefaultTerminal;

pub struct App {
    plugins: Option<Vec<String>>, // TODO: redefine plugins
}

impl App {
    pub fn new() -> App {
        App { plugins: None }
    }

    pub fn run(&mut self, term: &mut DefaultTerminal) -> io::Result<()> {
        Ok(())
    }
}
