use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use pixmux::boolpaths::BoolPathOperations;
use pixmux::{AdjustDirection, Tab, step_index};

use super::App;

#[derive(Default, Clone, Copy)]
pub enum Mode {
    #[default]
    Navigation,
    PaneEditing,
}

impl App {
    pub fn on_key_event(&mut self, key: KeyEvent) {
        match self.mode {
            Mode::Navigation => match (key.modifiers, key.code) {
                (KeyModifiers::NONE, KeyCode::Char('q') | KeyCode::Char('Q')) => self.quit(),

                (KeyModifiers::NONE, KeyCode::Tab) => {
                    self.current_tab = self.current_tab.cycle();
                }

                //
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

                //
                (KeyModifiers::NONE, KeyCode::Right) => {
                    if let Tab::Image = self.current_tab {
                        self.current_pane_path =
                            Some(self.pane_tree.next(&self.current_pane_path).unwrap());
                    }
                }
                (KeyModifiers::NONE, KeyCode::Left) => {
                    if let Tab::Image = self.current_tab {
                        self.current_pane_path =
                            Some(self.pane_tree.prev(&self.current_pane_path).unwrap());
                    }
                }

                //
                (KeyModifiers::NONE, KeyCode::Char('e')) => {
                    if let Tab::Image = self.current_tab
                        && self.current_pane_path.is_some()
                    {
                        self.mode = Mode::PaneEditing;
                    }
                }
                _ => {}
            },
            Mode::PaneEditing => match (key.modifiers, key.code) {
                (KeyModifiers::NONE, KeyCode::Esc) => self.mode = Mode::Navigation,
                (KeyModifiers::NONE, KeyCode::Char('v')) => {
                    if self.current_pane_path.is_none() {
                        return;
                    }
                    self.pane_tree
                        .vsplit(self.current_pane_path.as_ref().unwrap())
                        .unwrap();
                    if let Some(ref mut path) = self.current_pane_path {
                        path.push(false);
                    }
                }
                (KeyModifiers::NONE, KeyCode::Char('s')) => {
                    if self.current_pane_path.is_none() {
                        return;
                    }
                    self.pane_tree
                        .hsplit(self.current_pane_path.as_ref().unwrap())
                        .unwrap();
                    if let Some(ref mut path) = self.current_pane_path {
                        path.push(false);
                    }
                }
                (KeyModifiers::NONE, KeyCode::Char('x')) => {
                    if self.current_pane_path.is_none() {
                        return;
                    }
                    let new_path = BoolPathOperations::get_nearest_promoted_leaf(
                        &self.pane_tree.get_spec().leaf_paths,
                        self.current_pane_path.as_ref().unwrap(),
                    );

                    self.pane_tree
                        .remove(self.current_pane_path.as_ref().unwrap())
                        .unwrap();

                    self.current_pane_path = Some(new_path);
                }
                _ => {}
            },
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
