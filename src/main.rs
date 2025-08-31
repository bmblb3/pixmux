use color_eyre::Result;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use ratatui::{
    DefaultTerminal, Frame,
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    widgets::{Block, Paragraph, Tabs},
};

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    let terminal = ratatui::init();
    let result = App::new().run(terminal);
    ratatui::restore();
    result
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Tab {
    Data,
    Image,
}

impl Tab {
    fn title(&self) -> &'static str {
        match self {
            Tab::Data => "Data",
            Tab::Image => "Image",
        }
    }

    fn next(&self) -> Self {
        match self {
            Tab::Data => Tab::Image,
            Tab::Image => Tab::Data,
        }
    }

    fn previous(&self) -> Self {
        self.next()
    }
}

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

        let tab_titles = vec![Tab::Data.title(), Tab::Image.title()];
        let selected_tab = match self.current_tab {
            Tab::Data => 0,
            Tab::Image => 1,
        };

        let tabs = Tabs::new(tab_titles)
            .block(Block::bordered())
            .select(selected_tab)
            .highlight_style(Style::default().fg(Color::Yellow));

        frame.render_widget(tabs, chunks[0]);

        let content = match self.current_tab {
            Tab::Data => "Data content here",
            Tab::Image => "Image content here",
        };

        let paragraph =
            Paragraph::new(content).block(Block::bordered().title(self.current_tab.title()));

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_app_creation() {
        let app = App::new();
        assert_eq!(app.current_tab(), Tab::Data);
        assert!(!app.running);
    }

    #[test]
    fn test_tab_switching() {
        let mut app = App::new();
        assert_eq!(app.current_tab(), Tab::Data);

        app.next_tab();
        assert_eq!(app.current_tab(), Tab::Image);

        app.next_tab();
        assert_eq!(app.current_tab(), Tab::Data);
    }

    #[test]
    fn test_tab_titles() {
        let mut app = App::new();
        assert_eq!(app.current_tab().title(), "Data");

        app.next_tab();
        assert_eq!(app.current_tab().title(), "Image");
    }

    #[test]
    fn test_set_tab() {
        let mut app = App::new();
        app.set_tab(Tab::Image);
        assert_eq!(app.current_tab().title(), "Image");
    }
}
