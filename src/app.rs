use color_eyre::Result;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use ratatui::{
    DefaultTerminal, Frame,
    layout::{Constraint, Direction, Layout},
};

use crate::tab::Tab;

#[derive(Debug, Default)]
pub struct App {
    running: bool,
    current_tab: Tab,
    pub headers: Vec<String>,
    pub table: Vec<Vec<String>>,
    pub current_row_index: u16,
}

impl App {
    pub fn new() -> Self {
        Self {
            running: false,
            current_tab: Tab::Data,
            headers: vec!["Col1".into(), "Col2".into(), "Col3".into(), "Col4".into()],
            table: vec![
                vec!["R1C1".into(), "R1C2".into(), "R1C3".into(), "R1C4".into()],
                vec!["R2C1".into(), "R2C2".into(), "R2C3".into(), "R2C4".into()],
                vec!["R3C1".into(), "R3C2".into(), "R3C3".into(), "R3C4".into()],
                vec!["R4C1".into(), "R4C2".into(), "R4C3".into(), "R4C4".into()],
                vec!["R5C1".into(), "R5C2".into(), "R5C3".into(), "R5C4".into()],
            ],
            current_row_index: 0,
        }
    }

    pub fn next_tab(&mut self) {
        self.current_tab = self.current_tab.next();
    }

    pub fn previous_tab(&mut self) {
        self.current_tab = self.current_tab.previous();
    }

    pub fn next_row(&mut self) {
        if self.current_row_index < (self.table.len() as u16 - 1) {
            self.current_row_index += 1;
        }
    }

    pub fn prev_row(&mut self) {
        if self.current_row_index > 0 {
            self.current_row_index -= 1;
        }
    }

    pub fn run(mut self, mut terminal: DefaultTerminal) -> Result<()> {
        self.running = true;
        while self.running {
            terminal.draw(|frame| self.render(frame))?;
            self.handle_crossterm_events()?;
        }
        Ok(())
    }

    fn render(&mut self, frame: &mut Frame) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(3), Constraint::Min(0)])
            .split(frame.area());

        self.current_tab.render_navbar(frame, chunks[0]);
        self.current_tab.render(frame, chunks[1], self);
    }

    fn handle_crossterm_events(&mut self) -> Result<()> {
        match event::read()? {
            Event::Key(key) if key.kind == KeyEventKind::Press => self.on_key_event(key),
            Event::Mouse(_) => {}
            Event::Resize(_, _) => {}
            _ => {}
        }
        Ok(())
    }

    fn on_key_event(&mut self, key: KeyEvent) {
        match (key.modifiers, key.code) {
            (_, KeyCode::Esc | KeyCode::Char('q'))
            | (KeyModifiers::CONTROL, KeyCode::Char('c') | KeyCode::Char('C')) => self.quit(),
            (_, KeyCode::Tab) => self.next_tab(),
            (KeyModifiers::SHIFT, KeyCode::BackTab) => self.previous_tab(),
            (_, KeyCode::Up) | (_, KeyCode::Down) => self.handle_updown(key.code),
            _ => {}
        }
    }

    fn handle_updown(&mut self, code: KeyCode) {
        match self.current_tab {
            Tab::Data => match code {
                KeyCode::Down => self.next_row(),
                KeyCode::Up => self.prev_row(),
                _ => {}
            },
            Tab::Image => {}
        }
    }

    fn quit(&mut self) {
        self.running = false;
    }
}
