use color_eyre::Result;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use ratatui::{
    DefaultTerminal, Frame,
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    widgets::{Block, Paragraph, Tabs},
};

use crate::tab::Tab;

#[derive(Debug)]
pub struct App {
    running: bool,
    current_tab: Tab,
}

impl Default for App {
    fn default() -> Self {
        Self {
            running: false,
            current_tab: Tab::Data,
        }
    }
}

impl App {
    pub fn new() -> Self {
        Self::default()
    }

    #[cfg(test)]
    pub fn current_tab(&self) -> Tab {
        self.current_tab
    }

    pub fn set_tab(&mut self, tab: Tab) {
        self.current_tab = tab;
    }

    pub fn next_tab(&mut self) {
        self.set_tab(self.current_tab.next());
    }

    pub fn previous_tab(&mut self) {
        self.set_tab(self.current_tab.previous());
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

        let tabs = Tabs::new(Tab::titles())
            .block(Block::bordered())
            .select(self.current_tab.to_index())
            .highlight_style(Style::default().fg(Color::Yellow));

        frame.render_widget(tabs, chunks[0]);

        let paragraph = Paragraph::new(self.current_tab.content())
            .block(Block::bordered().title(self.current_tab.title()));

        frame.render_widget(paragraph, chunks[1]);
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
            _ => {}
        }
    }

    fn quit(&mut self) {
        self.running = false;
    }
}
