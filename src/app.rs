use std::{fs::File, io::Read};

use color_eyre::Result;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use ratatui::{
    DefaultTerminal, Frame,
    layout::{Constraint, Direction, Layout},
};

use crate::{image_layout::Pane, tab::Tab};

#[derive(Debug, Default)]
pub struct App {
    running: bool,
    current_tab: Tab,
    pub col_headers: Vec<String>,
    pub table_rows: Vec<Vec<String>>,
    pub current_row_index: u16,
    pub root_imgpane: Pane,
    pub current_imgpane_id: usize,
}

impl App {
    pub fn new(csv_path: &str) -> Result<Self> {
        let (headers, table) = Self::read_csv(csv_path)?;
        Ok(Self {
            running: false,
            current_tab: Tab::Data,
            col_headers: headers,
            table_rows: table,
            current_row_index: 0,
            current_imgpane_id: 0,
            root_imgpane: Pane::Split {
                direction: Direction::Vertical,
                first: Box::new(Pane::Split {
                    direction: Direction::Horizontal,
                    first: Box::new(Pane::Leaf),
                    second: Box::new(Pane::Split {
                        direction: Direction::Vertical,
                        first: Box::new(Pane::Leaf),
                        second: Box::new(Pane::Leaf),
                    }),
                }),
                second: Box::new(Pane::Leaf),
            },
        })
    }

    fn read_csv(path: &str) -> Result<(Vec<String>, Vec<Vec<String>>)> {
        let mut file = File::open(path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;

        let mut lines = contents.lines();
        let headers = lines
            .next()
            .ok_or_else(|| color_eyre::eyre::eyre!("Empty CSV file"))?
            .split(',')
            .map(|s| s.trim().to_string())
            .collect();

        let table = lines
            .map(|line| line.split(',').map(|s| s.trim().to_string()).collect())
            .collect();

        Ok((headers, table))
    }

    pub fn next_tab(&mut self) {
        self.current_tab = self.current_tab.next();
    }

    pub fn next_row(&mut self) {
        if self.current_row_index < (self.table_rows.len() as u16 - 1) {
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
            //
            (_, KeyCode::Esc | KeyCode::Char('q'))
            | (KeyModifiers::CONTROL, KeyCode::Char('c') | KeyCode::Char('C')) => self.quit(),
            //
            (_, KeyCode::Tab | KeyCode::BackTab) => self.next_tab(),
            (_, KeyCode::Up) | (_, KeyCode::Down) => self.handle_updown(key.code),
            //
            (_, KeyCode::Char('n')) => match self.current_tab {
                Tab::Image => self.cycle_imagepane(CycleDirection::Forward),
                Tab::Data => {}
            },
            (KeyModifiers::SHIFT, KeyCode::Char('N')) => match self.current_tab {
                Tab::Image => self.cycle_imagepane(CycleDirection::Backward),
                Tab::Data => {}
            },
            _ => {}
        }
    }

    fn handle_updown(&mut self, code: KeyCode) {
        match code {
            KeyCode::Down => self.next_row(),
            KeyCode::Up => self.prev_row(),
            _ => {}
        }
    }

    fn quit(&mut self) {
        self.running = false;
    }

    fn cycle_imagepane(&mut self, dir: CycleDirection) {
        let mut pane_count = 0;
        Self::get_total_imgpanes(&self.root_imgpane, &mut pane_count);
        let delta = match dir {
            CycleDirection::Forward => 1,
            CycleDirection::Backward => pane_count - 1,
        };
        self.current_imgpane_id += delta as usize;
        self.current_imgpane_id %= pane_count as usize;
    }

    fn get_total_imgpanes(_pane: &Pane, _counter: &mut u16) {
        match _pane {
            Pane::Leaf => *_counter += 1,
            Pane::Split { first, second, .. } => {
                Self::get_total_imgpanes(first, _counter);
                Self::get_total_imgpanes(second, _counter);
            }
        }
    }
}

pub enum CycleDirection {
    Forward,
    Backward,
}
