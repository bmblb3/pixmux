use std::path;

use color_eyre::Result;
use pixmux::panes::Pane;
use pixmux::tab::Tab;
use ratatui::layout::{Constraint, Direction, Layout};
use ratatui::{DefaultTerminal, Frame};

use crate::app::events::Mode;
use crate::ui;

pub mod events;

#[derive(Default)]
pub struct App {
    running: bool,
    pub col_headers: Vec<String>,
    pub current_datarow_index: usize,
    pub current_pane_path: Vec<bool>,
    pub current_tab: Tab,
    pub imagedir_paths: Vec<std::path::PathBuf>,
    pub mode: events::Mode,
    pub pane_tree: Pane,
    pub table_rows: Vec<Vec<String>>,
}

impl App {
    pub fn new(csv_path: path::PathBuf) -> Result<Self> {
        let (col_headers, table_rows, imagedir_paths) = pixmux::parse_csv(&csv_path)?;
        Ok(Self {
            running: false,
            col_headers,
            current_datarow_index: 0,
            current_pane_path: vec![],
            current_tab: Tab::default(),
            imagedir_paths,
            mode: Mode::Navigation,
            pane_tree: Pane::default(),
            table_rows,
        })
    }

    fn quit(&mut self) {
        self.running = false;
    }

    fn render(&mut self, frame: &mut Frame) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(1),
                Constraint::Min(0),
                Constraint::Length(1),
            ])
            .split(frame.area());

        ui::NavBarUI::render(frame, chunks[0], self);

        match self.current_tab {
            pixmux::Tab::Data => {
                frame.render_widget(ui::TableTabUI::create_widget(self), chunks[1]);
            }
            pixmux::Tab::Image => {
                ui::ImageTabUI::render(frame, chunks[1], self);
            }
        }

        ui::Footer::render(frame, chunks[2], self);
    }

    pub fn run(mut self, mut terminal: DefaultTerminal) -> Result<()> {
        self.running = true;
        while self.running {
            terminal.draw(|frame| self.render(frame))?;
            self.handle_crossterm_events()?;
        }
        Ok(())
    }
}
