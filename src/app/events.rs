use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use pixmux::boolpaths::BoolPathOperations;
use pixmux::{AdjustDirection, Tab, step_index};

use super::App;

#[derive(Default, Clone, Copy)]
pub enum Mode {
    #[default]
    Navigation,
    Layout,
    Image,
}

impl App {
    pub fn on_key_event(&mut self, key: KeyEvent) {
        match (self.mode, &self.current_tab) {
            (Mode::Navigation, Tab::Data) => match (key.modifiers, key.code) {
                (KeyModifiers::NONE, KeyCode::Char('q') | KeyCode::Char('Q')) => self.quit(),

                (KeyModifiers::NONE, KeyCode::Tab) => {
                    self.current_tab = self.current_tab.cycle();
                }

                (KeyModifiers::NONE, KeyCode::Up) => {
                    self.current_datarow_index = step_index(
                        self.current_datarow_index,
                        self.table_rows.len(),
                        AdjustDirection::Previous,
                    )
                }
                (KeyModifiers::NONE, KeyCode::Down) => {
                    self.current_datarow_index = step_index(
                        self.current_datarow_index,
                        self.table_rows.len(),
                        AdjustDirection::Next,
                    )
                }

                _ => {}
            },

            (Mode::Navigation, Tab::Image) => match (key.modifiers, key.code) {
                (KeyModifiers::NONE, KeyCode::Char('q') | KeyCode::Char('Q')) => self.quit(),

                (KeyModifiers::NONE, KeyCode::Tab) => {
                    self.current_tab = self.current_tab.cycle();
                }

                (KeyModifiers::NONE, KeyCode::Left) => {
                    self.current_pane_path = self.pane_tree.prev(&self.current_pane_path).unwrap();
                    // TODO: HPrev
                }
                (KeyModifiers::NONE, KeyCode::Up) => {
                    self.current_pane_path = self.pane_tree.prev(&self.current_pane_path).unwrap();
                    // TODO: VPrev
                }
                (KeyModifiers::NONE, KeyCode::Right) => {
                    self.current_pane_path = self.pane_tree.next(&self.current_pane_path).unwrap();
                    // TODO: HNext
                }
                (KeyModifiers::NONE, KeyCode::Down) => {
                    self.current_pane_path = self.pane_tree.prev(&self.current_pane_path).unwrap();
                    // TODO: VNext
                }

                (KeyModifiers::NONE, KeyCode::Char('l') | KeyCode::Char('L')) => {
                    self.mode = Mode::Layout;
                }

                (KeyModifiers::NONE, KeyCode::Char('i') | KeyCode::Char('I')) => {
                    self.mode = Mode::Image;
                }

                _ => {}
            },

            (Mode::Layout, Tab::Image) => match (key.modifiers, key.code) {
                (KeyModifiers::NONE, KeyCode::Esc) => self.mode = Mode::Navigation,

                (KeyModifiers::NONE, KeyCode::Char('v') | KeyCode::Char('V')) => {
                    self.pane_tree.vsplit(&self.current_pane_path).unwrap();
                    self.current_pane_path.push(false);
                }

                (KeyModifiers::NONE, KeyCode::Char('h') | KeyCode::Char('H')) => {
                    self.pane_tree.hsplit(&self.current_pane_path).unwrap();
                    self.current_pane_path.push(false);
                }

                (KeyModifiers::NONE, KeyCode::Char('x') | KeyCode::Char('X')) => {
                    let new_path = BoolPathOperations::get_nearest_promoted_leaf(
                        &self.pane_tree.get_spec().leaf_paths,
                        &self.current_pane_path,
                    );
                    self.pane_tree.remove(&self.current_pane_path).unwrap();
                    self.current_pane_path = new_path;
                }

                (KeyModifiers::NONE, KeyCode::Left) => {
                    // TODO: Resize pane
                }
                (KeyModifiers::NONE, KeyCode::Up) => {
                    // TODO: Resize pane
                }
                (KeyModifiers::NONE, KeyCode::Right) => {
                    // TODO: Resize pane
                }
                (KeyModifiers::NONE, KeyCode::Down) => {
                    // TODO: Resize pane
                }

                _ => {}
            },

            (Mode::Image, Tab::Image) => match (key.modifiers, key.code) {
                (KeyModifiers::NONE, KeyCode::Esc) => self.mode = Mode::Navigation,

                (KeyModifiers::NONE, KeyCode::Char('n') | KeyCode::Char('N')) => {
                    // TODO: Next image
                }

                (KeyModifiers::NONE, KeyCode::Left) => {
                    // TODO: Pane image
                }
                (KeyModifiers::NONE, KeyCode::Up) => {
                    // TODO: Pane image
                }
                (KeyModifiers::NONE, KeyCode::Right) => {
                    // TODO: Pane image
                }
                (KeyModifiers::NONE, KeyCode::Down) => {
                    // TODO: Pane image
                }

                (KeyModifiers::NONE, KeyCode::Char('+')) => {
                    // TODO: Zoom
                }
                (KeyModifiers::NONE, KeyCode::Char('-')) => {
                    // TODO: Zoom
                }

                _ => {}
            },

            _ => {}
        }
    }

    pub fn handle_crossterm_events(&mut self) -> color_eyre::Result<()> {
        match event::read()? {
            Event::Key(key) if key.kind == KeyEventKind::Press => self.on_key_event(key),
            Event::Mouse(_) => {}
            Event::Resize(_, _) => {}
            _ => {}
        }
        Ok(())
    }
}
