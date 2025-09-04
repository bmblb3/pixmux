use color_eyre::Result;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use ratatui::layout::Direction;

use super::{App, CycleDirection};
use crate::tab::Tab;

impl App {
    pub fn handle_crossterm_events(&mut self) -> Result<()> {
        match event::read()? {
            Event::Key(key) if key.kind == KeyEventKind::Press => self.on_key_event(key),
            Event::Mouse(_) => {}
            Event::Resize(_, _) => {}
            _ => {}
        }
        Ok(())
    }

    pub fn on_key_event(&mut self, key: KeyEvent) {
        match (key.modifiers, key.code) {
            //
            (_, KeyCode::Esc | KeyCode::Char('q'))
            | (KeyModifiers::CONTROL, KeyCode::Char('c') | KeyCode::Char('C')) => self.quit(),
            //
            (KeyModifiers::NONE, KeyCode::Tab | KeyCode::BackTab) => self.next_tab(),
            (KeyModifiers::NONE, KeyCode::Up | KeyCode::Down) => self.handle_updown(key.code),
            //
            (KeyModifiers::NONE, KeyCode::Char('n')) => match self.current_tab {
                Tab::Image => self.cycle_imagepane(CycleDirection::Forward),
                Tab::Data => {}
            },
            (KeyModifiers::SHIFT, KeyCode::Char('N')) => match self.current_tab {
                Tab::Image => self.cycle_imagepane(CycleDirection::Backward),
                Tab::Data => {}
            },
            //
            (KeyModifiers::ALT, KeyCode::Char('v')) => match self.current_tab {
                Tab::Image => self.split_imgpane(Direction::Horizontal),
                Tab::Data => {}
            },
            (KeyModifiers::ALT, KeyCode::Char('s')) => match self.current_tab {
                Tab::Image => self.split_imgpane(Direction::Vertical),
                Tab::Data => {}
            },
            //
            (KeyModifiers::ALT, KeyCode::Left) => match self.current_tab {
                Tab::Image => self.resize_imgpane(-5, Direction::Horizontal),
                Tab::Data => {}
            },
            (KeyModifiers::ALT, KeyCode::Right) => match self.current_tab {
                Tab::Image => self.resize_imgpane(5, Direction::Horizontal),
                Tab::Data => {}
            },
            (KeyModifiers::ALT, KeyCode::Up) => match self.current_tab {
                Tab::Image => self.resize_imgpane(-5, Direction::Vertical),
                Tab::Data => {}
            },
            (KeyModifiers::ALT, KeyCode::Down) => match self.current_tab {
                Tab::Image => self.resize_imgpane(5, Direction::Vertical),
                Tab::Data => {}
            },
            //
            (KeyModifiers::ALT, KeyCode::Char('x')) => match self.current_tab {
                Tab::Image => self.remove_imgpane(),
                Tab::Data => {}
            },
            //
            (_, KeyCode::Char('d')) => match self.current_tab {
                Tab::Image => self.next_img(),
                Tab::Data => {}
            },
            _ => {}
        }
    }
}
