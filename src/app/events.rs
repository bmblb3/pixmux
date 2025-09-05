use color_eyre::Result;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use ratatui::layout;

use super::{App, Tab};

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
            (_, KeyCode::Char('q')) => self.quit(),

            //
            (KeyModifiers::NONE, KeyCode::Tab | KeyCode::BackTab) => self.next_tab(),

            //
            (KeyModifiers::NONE, KeyCode::Up) => {
                self.current_row_index = pixmux::step_index(
                    self.current_row_index,
                    self.table_rows.len(),
                    pixmux::AdjustDirection::Backward,
                )
            }
            (KeyModifiers::NONE, KeyCode::Down) => {
                self.current_row_index = pixmux::step_index(
                    self.current_row_index,
                    self.table_rows.len(),
                    pixmux::AdjustDirection::Forward,
                )
            }

            //
            (KeyModifiers::NONE, KeyCode::Char('n')) => match self.current_tab {
                Tab::Image => {
                    self.current_pane_path = self
                        .pane_tree
                        .cycle(&self.current_pane_path, pixmux::AdjustDirection::Forward)
                        .unwrap();
                }
                Tab::Data => {}
            },
            (KeyModifiers::SHIFT, KeyCode::Char('N')) => match self.current_tab {
                Tab::Image => {
                    self.current_pane_path = self
                        .pane_tree
                        .cycle(&self.current_pane_path, pixmux::AdjustDirection::Backward)
                        .unwrap();
                }
                Tab::Data => {}
            },

            //
            (KeyModifiers::CONTROL, KeyCode::Char('v')) => match self.current_tab {
                Tab::Image => {
                    self.current_pane_path = self
                        .pane_tree
                        .split_leaf_at(&self.current_pane_path, layout::Direction::Horizontal)
                        .unwrap();
                }
                Tab::Data => {}
            },
            (KeyModifiers::CONTROL, KeyCode::Char('s')) => match self.current_tab {
                Tab::Image => {
                    self.current_pane_path = self
                        .pane_tree
                        .split_leaf_at(&self.current_pane_path, layout::Direction::Vertical)
                        .unwrap();
                }
                Tab::Data => {}
            },

            //
            (KeyModifiers::CONTROL, KeyCode::Char('x')) => match self.current_tab {
                Tab::Image => {
                    self.current_pane_path = self
                        .pane_tree
                        .remove_leaf_at(&self.current_pane_path)
                        .unwrap();
                }
                Tab::Data => {}
            },

            //
            (KeyModifiers::CONTROL, KeyCode::Left) => match self.current_tab {
                Tab::Image => {
                    self.pane_tree
                        .resize_leaf_at(&self.current_pane_path, layout::Direction::Horizontal, -5)
                        .unwrap();
                }
                Tab::Data => {}
            },
            (KeyModifiers::CONTROL, KeyCode::Right) => match self.current_tab {
                Tab::Image => {
                    self.pane_tree
                        .resize_leaf_at(&self.current_pane_path, layout::Direction::Horizontal, 5)
                        .unwrap();
                }
                Tab::Data => {}
            },
            (KeyModifiers::CONTROL, KeyCode::Up) => match self.current_tab {
                Tab::Image => {
                    self.pane_tree
                        .resize_leaf_at(&self.current_pane_path, layout::Direction::Vertical, -5)
                        .unwrap();
                }
                Tab::Data => {}
            },
            (KeyModifiers::CONTROL, KeyCode::Down) => match self.current_tab {
                Tab::Image => {
                    self.pane_tree
                        .resize_leaf_at(&self.current_pane_path, layout::Direction::Vertical, 5)
                        .unwrap();
                }
                Tab::Data => {}
            },

            // //
            // (_, KeyCode::Char('d')) => match self.current_tab {
            //     Tab::Image => self.next_img(),
            //     Tab::Data => {}
            // },

            //
            _ => {}
        }
    }
}
