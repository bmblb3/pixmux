use std::path;

use color_eyre::Result;
use pixmux::{Pane, Tab};
use ratatui::layout::{Constraint, Direction, Layout};
use ratatui::{DefaultTerminal, Frame};

use crate::ui;

mod events;

#[derive(Default)]
pub struct App {
    running: bool,
    pub col_headers: Vec<String>,
    pub table_rows: Vec<Vec<String>>,
    pub imagedir_paths: Vec<std::path::PathBuf>,
    pub imagefile_basenames: Vec<String>,
    pub current_tab: Tab,
    pub current_datarow_index: usize,
    pub pane_tree: pixmux::Pane,
    pub current_pane_path: Vec<bool>,
}

impl App {
    pub fn new(csv_path: path::PathBuf) -> Result<Self> {
        let (col_headers, table_rows, imagedir_paths) = pixmux::parse_csv(&csv_path)?;
        let imagefile_basenames = pixmux::imagefile::collect_basenames(&imagedir_paths);
        Ok(Self {
            running: false,
            col_headers,
            table_rows,
            imagedir_paths,
            imagefile_basenames,
            current_tab: Tab::default(),
            current_datarow_index: 0,
            pane_tree: Pane::default(),
            current_pane_path: vec![],
        })
    }

    pub fn run(mut self, mut terminal: DefaultTerminal) -> Result<()> {
        self.running = true;
        while self.running {
            terminal.draw(|frame| self.render(frame))?;
            self.handle_crossterm_events()?;
        }
        Ok(())
    }

    fn quit(&mut self) {
        self.running = false;
    }

    fn render(&mut self, frame: &mut Frame) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(3), Constraint::Min(0)])
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
    }
}
