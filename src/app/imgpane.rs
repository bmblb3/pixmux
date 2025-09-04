use ratatui::layout::Direction;

use super::App;

#[derive(Debug, Clone)]
pub enum Pane {
    Leaf {
        image_id: usize,
    },
    Split {
        direction: Direction,
        pct: u8,
        first: Box<Pane>,
        second: Box<Pane>,
    },
}

impl Default for Pane {
    fn default() -> Self {
        Self::Leaf { image_id: 0 }
    }
}

impl Pane {
    pub fn leaf() -> Self {
        Self::Leaf { image_id: 0 }
    }

    pub fn split(direction: Direction) -> Self {
        Self::Split {
            direction,
            pct: 50,
            first: Box::new(Pane::leaf()),
            second: Box::new(Pane::leaf()),
        }
    }
}

impl App {
    //
    pub fn get_total_imgpanes(pane: &Pane, counter: &mut usize) {
        match pane {
            Pane::Leaf { .. } => *counter += 1,
            Pane::Split { first, second, .. } => {
                Self::get_total_imgpanes(first, counter);
                Self::get_total_imgpanes(second, counter);
            }
        }
    }

    // SPLIT AN IMGPANE
    pub fn split_imgpane(&mut self, split_direction: Direction) {
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

    // RESIZE AN IMGPANE
    pub fn resize_imgpane(&mut self, delta: i8, resize_direction: Direction) {
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

    // REMOVE AN IMGPANE
    pub fn remove_imgpane(&mut self) {
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
