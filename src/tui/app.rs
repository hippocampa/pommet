use std::{error::Error, io};

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    DefaultTerminal, Frame,
    widgets::{Block, Paragraph},
};

use crate::plugins::{Plugin, apache::Apache, mariadb::Mariadb, php::PHP};

pub struct App {
    plugins: Vec<Box<dyn Plugin>>,
    root_dir: String,
    exit: bool,
}

impl App {
    pub fn new() -> Self {
        Self {
            plugins: vec![
                Box::new(Apache::new()),
                Box::new(PHP::new()),
                Box::new(Mariadb::new()),
            ],
            root_dir: "C:/pommet".to_string(),
            exit: false,
        }
    }

    pub fn ensure_installation(&mut self) -> Result<(), Box<dyn Error>> {
        for plugin in &mut self.plugins {
            // !fix me: check if the dir in every plugin exist => true/false
            if !plugin.is_installed() {
                plugin.install()?;
            }
        }
        Ok(())
    }

    fn exit(&mut self) {
        self.exit = true
    }

    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> Result<(), Box<dyn Error>> {
        // test
        self.plugins[2].toggle()?;
        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn draw(&self, frame: &mut Frame) {
        frame.render_widget(
            Paragraph::new("Hello world").block(Block::bordered()),
            frame.area(),
        );
    }

    fn handle_events(&mut self) -> io::Result<()> {
        match event::read()? {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.handle_key_event(key_event)
            }
            _ => {}
        }
        Ok(())
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('q') => self.exit(),
            _ => {}
        }
    }
}
