use std::{fs::File, io::Read};

use color_eyre::Result;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use ratatui::{
    DefaultTerminal, Frame,
    layout::{Constraint, Direction, Layout},
};

use crate::{image_layout::Pane, tab::Tab};

type CsvData = (Vec<String>, Vec<Vec<String>>, Vec<std::path::PathBuf>);

#[derive(Debug, Default)]
#[allow(dead_code)]
pub struct App {
    running: bool,
    current_tab: Tab,
    pub col_headers: Vec<String>,
    pub table_rows: Vec<Vec<String>>,
    pub imgdir_paths: Vec<std::path::PathBuf>,
    pub current_row_index: u16,
    pub root_imgpane: Pane,
    pub current_imgpane_id: usize,
}

impl App {
    pub fn new(csv_path: &str) -> Result<Self> {
        let (col_headers, table_rows, imgdir_paths) = Self::read_csv(csv_path)?;
        Ok(Self {
            running: false,
            current_tab: Tab::Data,
            col_headers,
            table_rows,
            imgdir_paths,
            current_row_index: 0,
            current_imgpane_id: 0,
            root_imgpane: Pane::Leaf,
        })
    }

    fn read_csv(path: &str) -> Result<CsvData> {
        let mut file = File::open(path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;

        let mut lines = contents.lines();
        let header_line = lines
            .next()
            .ok_or_else(|| color_eyre::eyre::eyre!("Empty CSV file"))?;

        let all_headers: Vec<&str> = header_line.split(',').map(|s| s.trim()).collect();
        let underscore_col_idx = all_headers.iter().position(|&h| h == "_");

        let headers: Vec<String> = all_headers
            .iter()
            .enumerate()
            .filter(|(i, _)| Some(*i) != underscore_col_idx)
            .map(|(_, &h)| h.to_string())
            .collect();

        let csv_dir = std::path::Path::new(path)
            .parent()
            .unwrap_or(std::path::Path::new("."));
        let mut dir_paths = Vec::new();
        let mut table = Vec::new();

        for line in lines {
            let row: Vec<&str> = line.split(',').map(|s| s.trim()).collect();

            if let Some(idx) = underscore_col_idx {
                if idx < row.len() {
                    let rel_path = row[idx];
                    let abs_path = csv_dir.join(rel_path);
                    dir_paths.push(abs_path);
                }
            }

            let filtered_row: Vec<String> = row
                .iter()
                .enumerate()
                .filter(|(i, _)| Some(*i) != underscore_col_idx)
                .map(|(_, &cell)| cell.to_string())
                .collect();
            table.push(filtered_row);
        }

        Ok((headers, table, dir_paths))
    }

    pub fn collect_image_basenames(&self) -> std::collections::BTreeSet<String> {
        let mut basenames = std::collections::BTreeSet::new();
        let image_extensions = ["jpg", "jpeg", "png", "gif", "bmp", "tiff", "webp"];

        for dir_path in &self.imgdir_paths {
            if let Ok(entries) = std::fs::read_dir(dir_path) {
                for entry in entries.flatten() {
                    let path = entry.path();
                    if path.is_file() {
                        if let Some(ext) = path.extension() {
                            if let Some(ext_str) = ext.to_str() {
                                if image_extensions.contains(&ext_str.to_lowercase().as_str()) {
                                    if let Some(basename) = path.file_stem() {
                                        if let Some(basename_str) = basename.to_str() {
                                            basenames.insert(basename_str.to_string());
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        basenames
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
            Pane::Leaf => {
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
            Pane::Leaf => {
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
            Pane::Leaf => {
                let found_leaf = candidate_imgpane_id == target_imgpane_id;
                *candidate_imgpane_id += 1;
                (found_leaf, false)
            }
        }
    }
}

pub enum CycleDirection {
    Forward,
    Backward,
}
