use std::path::{self, PathBuf};

use color_eyre::Result;
use crossterm::event::KeyCode;
use ratatui::layout::{Constraint, Direction, Layout};
use ratatui::{DefaultTerminal, Frame};

use crate::image_layout::Pane;
use crate::tab::Tab;
use crate::utils::{self, CycleDirection, cycle_index};

mod events;

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
        let (col_headers, table_rows, imgdir_paths) = utils::parse_csv(&csv_path)?;
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
        utils::collect_imgfile_basenames(&self.imgdir_paths)
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

    pub fn next_row(&mut self) {
        if self.current_row_index < (self.table_rows.len() - 1) {
            self.current_row_index += 1;
        }
    }

    pub fn prev_row(&mut self) {
        if self.current_row_index > 0 {
            self.current_row_index -= 1;
        }
    }

    fn handle_updown(&mut self, code: KeyCode) {
        match code {
            KeyCode::Down => self.next_row(),
            KeyCode::Up => self.prev_row(),
            _ => {}
        }
    }

    fn cycle_imagepane(&mut self, dir: CycleDirection) {
        let mut pane_count = 0usize;
        Self::get_total_imgpanes(&self.root_imgpane, &mut pane_count);
        self.current_imgpane_id = cycle_index(self.current_imgpane_id, pane_count, dir);
    }

    fn set_img_impl(
        pane: &mut Pane,
        target_imgpane_id: &usize,
        candidate_imgpane_id: &mut usize,
        nr_images: &usize,
        cycle_direction: &CycleDirection,
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
                    CycleDirection::Forward => {
                        *image_id += 1;
                        *image_id %= nr_images;
                    }
                    CycleDirection::Backward => {
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
        let nr_images = utils::collect_imgfile_basenames(&self.imgdir_paths).len();
        Self::set_img_impl(
            &mut self.root_imgpane,
            &self.current_imgpane_id,
            &mut candidate_imgpane_id,
            &nr_images,
            &CycleDirection::Forward,
        );
    }

    fn get_total_imgpanes(pane: &Pane, counter: &mut usize) {
        match pane {
            Pane::Leaf { .. } => *counter += 1,
            Pane::Split { first, second, .. } => {
                Self::get_total_imgpanes(first, counter);
                Self::get_total_imgpanes(second, counter);
            }
        }
    }

    fn split_imgpane(&mut self, split_direction: Direction) {
        let mut candidate_imgpane_id = 0;
        Self::split_imgpane_impl(
            &mut self.root_imgpane,
            &self.current_imgpane_id,
            &mut candidate_imgpane_id,
            &split_direction,
        );
    }

    fn split_imgpane_impl(
        pane: &mut Pane,
        target_imgpane_id: &usize,
        candidate_imgpane_id: &mut usize,
        split_direction: &Direction,
    ) -> bool {
        match pane {
            Pane::Split { first, second, .. } => {
                if Self::split_imgpane_impl(
                    first,
                    target_imgpane_id,
                    candidate_imgpane_id,
                    split_direction,
                ) {
                    return true;
                }

                if Self::split_imgpane_impl(
                    second,
                    target_imgpane_id,
                    candidate_imgpane_id,
                    split_direction,
                ) {
                    return true;
                }

                false
            }
            Pane::Leaf { .. } => {
                if candidate_imgpane_id != target_imgpane_id {
                    *candidate_imgpane_id += 1;
                    return false;
                }
                *pane = Pane::split(*split_direction);
                true
            }
        }
    }

    fn resize_imgpane(&mut self, delta: i8, resize_direction: Direction) {
        let mut candidate_imgpane_id = 0;
        Self::resize_imgpane_impl(
            &mut self.root_imgpane,
            &self.current_imgpane_id,
            &mut candidate_imgpane_id,
            &delta,
            &resize_direction,
        );
    }

    fn resize_imgpane_impl(
        pane: &mut Pane,
        target_imgpane_id: &usize,
        candidate_imgpane_id: &mut usize,
        delta: &i8,
        resize_direction: &Direction,
    ) -> (bool, bool) // found_leaf, resized
    {
        match pane {
            Pane::Split {
                direction,
                pct,
                first,
                second,
                ..
            } => {
                let (first_found_leaf, first_resized) = Self::resize_imgpane_impl(
                    first,
                    target_imgpane_id,
                    candidate_imgpane_id,
                    delta,
                    resize_direction,
                );
                if first_resized {
                    return (first_found_leaf, first_resized);
                }
                if first_found_leaf && direction == resize_direction {
                    *pct = ((*pct as i8) + delta).clamp(5, 95) as u8;
                    return (first_found_leaf, true);
                }

                let (second_found_leaf, second_resized) = Self::resize_imgpane_impl(
                    second,
                    target_imgpane_id,
                    candidate_imgpane_id,
                    delta,
                    resize_direction,
                );
                if second_resized {
                    return (second_found_leaf, second_resized);
                }
                if second_found_leaf && direction == resize_direction {
                    *pct = ((*pct as i8) + delta).clamp(5, 95) as u8;
                    return (second_found_leaf, true);
                }

                (
                    first_found_leaf || second_found_leaf,
                    first_resized || second_resized,
                )
            }
            Pane::Leaf { .. } => {
                let found_leaf = candidate_imgpane_id == target_imgpane_id;
                *candidate_imgpane_id += 1;
                (found_leaf, false)
            }
        }
    }

    fn remove_imgpane(&mut self) {
        let mut candidate_imgpane_id = 0;
        Self::remove_imgpane_impl(
            &mut self.root_imgpane,
            &self.current_imgpane_id,
            &mut candidate_imgpane_id,
        );
    }

    fn remove_imgpane_impl(
        pane: &mut Pane,
        target_imgpane_id: &usize,
        candidate_imgpane_id: &mut usize,
    ) -> (bool, bool) // found_leaf, removed
    {
        match pane {
            Pane::Split { first, second, .. } => {
                let (first_found_leaf, first_removed) =
                    Self::remove_imgpane_impl(first, target_imgpane_id, candidate_imgpane_id);
                if first_removed {
                    return (first_found_leaf, first_removed);
                }
                if first_found_leaf {
                    *pane = *second.clone();
                    return (first_found_leaf, true);
                }

                let (second_found_leaf, second_removed) =
                    Self::remove_imgpane_impl(second, target_imgpane_id, candidate_imgpane_id);
                if second_removed {
                    return (second_found_leaf, second_removed);
                }
                if second_found_leaf {
                    *pane = *first.clone();
                    return (second_found_leaf, true);
                }

                (
                    first_found_leaf || second_found_leaf,
                    first_removed || second_removed,
                )
            }
            Pane::Leaf { .. } => {
                let found_leaf = candidate_imgpane_id == target_imgpane_id;
                *candidate_imgpane_id += 1;
                (found_leaf, false)
            }
        }
    }
}
