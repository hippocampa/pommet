use std::{error::Error, io};

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{DefaultTerminal, Frame};

use crate::plugins::{
    Plugin, PluginStatus, apache::Apache, mariadb::Mariadb, php::PHP, phpmyadmin::PMA,
};

use super::dashboard::Dashboard;

pub struct App {
    plugins: Vec<Box<dyn Plugin>>,
    exit: bool,
    selected_index: usize,
}

impl App {
    pub fn new() -> Self {
        Self {
            plugins: vec![
                Box::new(Apache::new()),
                Box::new(PHP::new()),
                Box::new(Mariadb::new()),
                Box::new(PMA::new()),
            ],
            exit: false,
            selected_index: 0,
        }
    }

    pub fn ensure_installation(&mut self) -> Result<(), Box<dyn Error>> {
        for plugin in &mut self.plugins {
            if !plugin.is_installed() {
                plugin.install()?;
            }
        }
        Ok(())
    }
    fn exit(&mut self) {
        self.shutdown_running_plugins();
        self.exit = true;
    }

    fn shutdown_running_plugins(&mut self) {
        for plugin in &mut self.plugins {
            if plugin.is_toggleable() {
                if let PluginStatus::On = plugin.status() {
                    if let Err(e) = plugin.toggle() {
                        eprintln!("Failed to stop plugin {}: {}", plugin.name(), e);
                    }
                }
            }
        }
    }

    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> Result<(), Box<dyn Error>> {
        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }
    fn draw(&self, frame: &mut Frame) {
        let dashboard = Dashboard {
            plugins: &self.plugins,
            selected_index: self.selected_index,
        };
        frame.render_widget(dashboard, frame.area());
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
            KeyCode::Up | KeyCode::Char('k') => self.move_up(),
            KeyCode::Down | KeyCode::Char('j') => self.move_down(),
            KeyCode::Char(' ') => self.toggle_selected_plugin(),
            _ => {}
        }
    }

    fn move_up(&mut self) {
        let toggleable_count = self.plugins.iter().filter(|p| p.is_toggleable()).count();
        if toggleable_count > 0 && self.selected_index > 0 {
            self.selected_index -= 1;
        }
    }

    fn move_down(&mut self) {
        let toggleable_count = self.plugins.iter().filter(|p| p.is_toggleable()).count();
        if toggleable_count > 0 && self.selected_index < toggleable_count - 1 {
            self.selected_index += 1;
        }
    }

    fn toggle_selected_plugin(&mut self) {
        let toggleable_plugins: Vec<usize> = self
            .plugins
            .iter()
            .enumerate()
            .filter(|(_, plugin)| plugin.is_toggleable())
            .map(|(i, _)| i)
            .collect();
        if let Some(&actual_index) = toggleable_plugins.get(self.selected_index) {
            if let Err(e) = self.plugins[actual_index].toggle() {
                eprintln!("Failed to toggle plugin: {}", e);
            }
        }
    }
}
