use std::path::{self, PathBuf};

use color_eyre::Result;
use ratatui::layout::{Constraint, Direction, Layout};
use ratatui::{DefaultTerminal, Frame};

use crate::app::imgpane::Pane;
use crate::ui::Tab;

mod events;
pub mod imgpane;

#[derive(Debug, Default)]
pub struct App {
    running: bool,
    current_tab: Tab,
    pub col_headers: Vec<String>,
    pub table_rows: Vec<Vec<String>>,
    pub imgdir_paths: Vec<std::path::PathBuf>,
    pub current_row_index: usize,
    pub root_imgpane: Pane,
    pub current_imgpane_id: usize,
}

impl App {
    pub fn new(csv_path: path::PathBuf) -> Result<Self> {
        let (col_headers, table_rows, imgdir_paths) = pixmux::parse_csv(&csv_path)?;
        Ok(Self {
            running: false,
            current_tab: Tab::Data,
            col_headers,
            table_rows,
            imgdir_paths,
            current_row_index: 0,
            current_imgpane_id: 0,
            root_imgpane: Pane::leaf(),
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

        self.current_tab.render_navbar(frame, chunks[0]);
        self.current_tab.render(frame, chunks[1], self);
    }

    pub fn get_basename(&self, index: &usize) -> Option<String> {
        pixmux::collect_imgfile_basenames(&self.imgdir_paths)
            .get(*index)
            .cloned()
    }

    pub fn get_imgdir_path(&self, index: &usize) -> &PathBuf {
        &self.imgdir_paths[*index]
    }

    pub fn get_fullimgpath(&self, image_index: &usize, row_index: &usize) -> Option<PathBuf> {
        let basename = self.get_basename(image_index)?;
        Some(self.get_imgdir_path(row_index).join(basename))
    }

    pub fn next_tab(&mut self) {
        self.current_tab = self.current_tab.next();
    }

    fn cycle_imagepane(&mut self, dir: pixmux::AdjustDirection) {
        let mut pane_count = 0usize;
        Self::get_total_imgpanes(&self.root_imgpane, &mut pane_count);
        self.current_imgpane_id = pixmux::cycle_index(self.current_imgpane_id, pane_count, dir);
    }

    fn set_img_impl(
        pane: &mut Pane,
        target_imgpane_id: &usize,
        candidate_imgpane_id: &mut usize,
        nr_images: &usize,
        cycle_direction: &pixmux::AdjustDirection,
    ) -> bool {
        match pane {
            Pane::Split { first, second, .. } => {
                if Self::set_img_impl(
                    first,
                    target_imgpane_id,
                    candidate_imgpane_id,
                    nr_images,
                    cycle_direction,
                ) {
                    return true;
                }

                if Self::set_img_impl(
                    second,
                    target_imgpane_id,
                    candidate_imgpane_id,
                    nr_images,
                    cycle_direction,
                ) {
                    return true;
                }

                false
            }
            Pane::Leaf { image_id } => {
                if candidate_imgpane_id != target_imgpane_id {
                    *candidate_imgpane_id += 1;
                    return false;
                }
                match cycle_direction {
                    pixmux::AdjustDirection::Forward => {
                        *image_id += 1;
                        *image_id %= nr_images;
                    }
                    pixmux::AdjustDirection::Backward => {
                        *image_id += nr_images - 1;
                        *image_id %= nr_images;
                    }
                }
                true
            }
        }
    }

    fn next_img(&mut self) {
        let mut candidate_imgpane_id = 0;
        let nr_images = pixmux::collect_imgfile_basenames(&self.imgdir_paths).len();
        Self::set_img_impl(
            &mut self.root_imgpane,
            &self.current_imgpane_id,
            &mut candidate_imgpane_id,
            &nr_images,
            &pixmux::AdjustDirection::Forward,
        );
    }
}
