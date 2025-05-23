use std::{collections::VecDeque, io};

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    DefaultTerminal, Frame,
    layout::{Constraint, Direction, Layout},
};

use crate::{
    plugins::{Plugin, apache::Apache, msql::MySQL},
    tui::{
        handlers::focushandler::FocusState,
        widgets::{header::Header, leftpanel::LeftPanel, rightpanel::RightPanel},
    },
};

pub struct App {
    plugins: Vec<Box<dyn Plugin>>,
    logs: VecDeque<String>,
    max_log_lines: usize,
    focus: FocusState,
    exit: bool,
}

impl App {
    pub fn new() -> App {
        App {
            plugins: vec![Box::new(Apache::new()), Box::new(MySQL::new())], // TODO: it's just a toy for now
            logs: VecDeque::new(),
            max_log_lines: 100,
            focus: FocusState::ControlPanel,
            exit: false,
        }
    }

    pub fn log_this(&mut self, text: &str) {
        if self.logs.len() >= self.max_log_lines {
            self.logs.pop_front();
        }
        self.logs.push_back(text.to_string());
    }

    pub fn exit(&mut self) {
        self.exit = true;
    }

    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn draw(&self, frame: &mut Frame) {
        //           Layout
        // ┌──────────────────────────────┐
        // │         Header               │
        // ├──────────────┬───────────────┤
        // │┌────────────┐│               │
        // ││  Cpanel    ││  Log          │
        // │├────────────┤│               │
        // ││ Help Text  ││               │
        // │└────────────┘└───────────────┘
        //
        let main_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![Constraint::Length(5), Constraint::Min(0)])
            .split(frame.area());
        frame.render_widget(Header::new(), main_layout[0]);

        let body_layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(vec![Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(main_layout[1]);

        frame.render_widget(
            LeftPanel::new(
                &self.plugins,
                if self.focus == FocusState::ControlPanel {
                    true
                } else {
                    false
                },
            ),
            body_layout[0],
        );
        frame.render_widget(RightPanel, body_layout[1]);
    }

    fn handle_events(&mut self) -> io::Result<()> {
        match event::read()? {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.handle_key_event(key_event);
            }
            _ => {}
        }
        Ok(())
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('q') => self.exit(),
            KeyCode::Char('1') => self.focus = FocusState::ControlPanel,
            KeyCode::Char('2') => self.focus = FocusState::LogPanel,
            _ => {}
        }
    }
}
