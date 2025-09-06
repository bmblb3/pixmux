use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use pixmux::AdjustDirection;
use ratatui::layout;

use super::{App, Tab};

impl App {
    pub fn handle_crossterm_events(&mut self) -> color_eyre::Result<()> {
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
            (KeyModifiers::NONE, KeyCode::Tab) => {
                self.current_tab = self.current_tab.cycle();
            }

            //
            (KeyModifiers::NONE, KeyCode::Up) => {
                self.current_datarow_index = pixmux::step_index(
                    self.current_datarow_index,
                    self.table_rows.len(),
                    pixmux::AdjustDirection::Previous,
                )
            }
            (KeyModifiers::NONE, KeyCode::Down) => {
                self.current_datarow_index = pixmux::step_index(
                    self.current_datarow_index,
                    self.table_rows.len(),
                    pixmux::AdjustDirection::Next,
                )
            }

            //
            (KeyModifiers::NONE, KeyCode::Char('h')) => match self.current_tab {
                Tab::Image => {
                    self.current_pane_path = self
                        .pane_tree
                        .navigate(
                            &self.current_pane_path,
                            layout::Direction::Horizontal,
                            pixmux::AdjustDirection::Previous,
                        )
                        .unwrap();
                }
                Tab::Data => {}
            },
            (KeyModifiers::NONE, KeyCode::Char('j')) => match self.current_tab {
                Tab::Image => {
                    self.current_pane_path = self
                        .pane_tree
                        .navigate(
                            &self.current_pane_path,
                            layout::Direction::Vertical,
                            pixmux::AdjustDirection::Next,
                        )
                        .unwrap();
                }
                Tab::Data => {}
            },
            (KeyModifiers::NONE, KeyCode::Char('k')) => match self.current_tab {
                Tab::Image => {
                    self.current_pane_path = self
                        .pane_tree
                        .navigate(
                            &self.current_pane_path,
                            layout::Direction::Vertical,
                            pixmux::AdjustDirection::Previous,
                        )
                        .unwrap();
                }
                Tab::Data => {}
            },
            (KeyModifiers::NONE, KeyCode::Char('l')) => match self.current_tab {
                Tab::Image => {
                    self.current_pane_path = self
                        .pane_tree
                        .navigate(
                            &self.current_pane_path,
                            layout::Direction::Horizontal,
                            pixmux::AdjustDirection::Next,
                        )
                        .unwrap();
                }
                Tab::Data => {}
            },

            //
            (KeyModifiers::NONE, KeyCode::Char('r')) => match self.current_tab {
                Tab::Image => {
                    self.current_pane_path = self
                        .pane_tree
                        .split_leaf_at(&self.current_pane_path, layout::Direction::Horizontal)
                        .unwrap();
                }
                Tab::Data => {}
            },
            (KeyModifiers::NONE, KeyCode::Char('b')) => match self.current_tab {
                Tab::Image => {
                    self.current_pane_path = self
                        .pane_tree
                        .split_leaf_at(&self.current_pane_path, layout::Direction::Vertical)
                        .unwrap();
                }
                Tab::Data => {}
            },

            //
            (KeyModifiers::NONE, KeyCode::Char('x')) => match self.current_tab {
                Tab::Image => {
                    self.current_pane_path = self
                        .pane_tree
                        .remove_leaf_at(&self.current_pane_path)
                        .unwrap();
                }
                Tab::Data => {}
            },

            //
            (KeyModifiers::NONE, KeyCode::Char('a')) => match self.current_tab {
                Tab::Image => {
                    self.pane_tree
                        .resize_leaf_at(&self.current_pane_path, layout::Direction::Horizontal, -5)
                        .unwrap();
                }
                Tab::Data => {}
            },
            (KeyModifiers::NONE, KeyCode::Char('s')) => match self.current_tab {
                Tab::Image => {
                    self.pane_tree
                        .resize_leaf_at(&self.current_pane_path, layout::Direction::Vertical, 5)
                        .unwrap();
                }
                Tab::Data => {}
            },
            (KeyModifiers::NONE, KeyCode::Char('w')) => match self.current_tab {
                Tab::Image => {
                    self.pane_tree
                        .resize_leaf_at(&self.current_pane_path, layout::Direction::Vertical, -5)
                        .unwrap();
                }
                Tab::Data => {}
            },
            (KeyModifiers::NONE, KeyCode::Char('d')) => match self.current_tab {
                Tab::Image => {
                    self.pane_tree
                        .resize_leaf_at(&self.current_pane_path, layout::Direction::Horizontal, 5)
                        .unwrap();
                }
                Tab::Data => {}
            },

            //
            (_, KeyCode::Char('ä')) => match self.current_tab {
                Tab::Image => self
                    .pane_tree
                    .cycle_image(
                        &self.current_pane_path,
                        self.imagefile_basenames.len(),
                        AdjustDirection::Next,
                    )
                    .unwrap(),
                Tab::Data => {}
            },
            (_, KeyCode::Char('ö')) => match self.current_tab {
                Tab::Image => self
                    .pane_tree
                    .cycle_image(
                        &self.current_pane_path,
                        self.imagefile_basenames.len(),
                        AdjustDirection::Previous,
                    )
                    .unwrap(),
                Tab::Data => {}
            },

            //
            _ => {}
        }
    }
}
