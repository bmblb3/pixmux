use ratatui::layout;

pub enum Pane {
    Leaf {
        image_id: usize,
    },
    Split {
        direction: layout::Direction,
        pct: u8,
        first: Box<Pane>,
        second: Box<Pane>,
    },
}

impl Pane {
    pub fn new_leaf() -> Self {
        Pane::Leaf { image_id: 0 }
    }

    pub fn new_split(direction: layout::Direction) -> Pane {
        Pane::Split {
            direction,
            pct: 50,
            first: Box::new(Self::new_leaf()),
            second: Box::new(Self::new_leaf()),
        }
    }

    fn collect_leaf_paths_impl(
        &self,
        current_path: &mut Vec<bool>,
        all_paths: &mut Vec<Vec<bool>>,
    ) {
        match self {
            Pane::Leaf { .. } => {
                all_paths.push(current_path.clone());
            }
            Pane::Split { first, second, .. } => {
                current_path.push(true);
                first.collect_leaf_paths_impl(current_path, all_paths);
                current_path.pop();

                current_path.push(false);
                second.collect_leaf_paths_impl(current_path, all_paths);
                current_path.pop();
            }
        }
    }

    pub fn collect_leaf_paths(&self) -> Vec<Vec<bool>> {
        let mut all_paths = Vec::new();
        let mut current_path = Vec::new();
        self.collect_leaf_paths_impl(&mut current_path, &mut all_paths);
        all_paths
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_leaf() {
        let tree = Pane::new_leaf();

        assert!(matches!(tree, Pane::Leaf { .. }));
    }

    #[test]
    fn test_new_split() {
        let tree = Pane::new_split(layout::Direction::Horizontal);

        assert!(matches!(
            tree,
            Pane::Split {
                direction: layout::Direction::Horizontal,
                ..
            }
        ));
    }

    #[test]
    fn test_new_split_get_leaves() {
        let tree = Pane::new_split(layout::Direction::Horizontal);
        let paths = tree.collect_leaf_paths();

        assert_eq!(paths, vec![vec![true], vec![false]]);
    }

    #[test]
    fn test_a1a2b_split_get_leaves() {
        let split1 = Pane::new_split(layout::Direction::Horizontal);
        let leaf = Pane::new_leaf();
        let tree = Pane::Split {
            direction: layout::Direction::Vertical,
            pct: 50,
            first: Box::new(split1),
            second: Box::new(leaf),
        };

        let paths = tree.collect_leaf_paths();

        assert_eq!(
            paths,
            vec![vec![true, true], vec![true, false], vec![false]]
        );
    }

    #[test]
    fn test_ab1b2_split_get_leaves() {
        let split1 = Pane::new_split(layout::Direction::Horizontal);
        let leaf = Pane::new_leaf();
        let tree = Pane::Split {
            direction: layout::Direction::Vertical,
            pct: 50,
            first: Box::new(leaf),
            second: Box::new(split1),
        };

        let paths = tree.collect_leaf_paths();

        assert_eq!(
            paths,
            vec![vec![true], vec![false, true], vec![false, false]]
        );
    }

    #[test]
    fn test_deep_nesting_path() {
        let inner_split = Pane::new_split(layout::Direction::Horizontal);
        let middle_split = Pane::Split {
            direction: layout::Direction::Vertical,
            pct: 50,
            first: Box::new(Pane::new_leaf()),
            second: Box::new(inner_split),
        };
        let outer_split = Pane::Split {
            direction: layout::Direction::Horizontal,
            pct: 50,
            first: Box::new(middle_split),
            second: Box::new(Pane::new_leaf()),
        };

        let paths = outer_split.collect_leaf_paths();

        assert_eq!(
            paths,
            vec![
                vec![true, true],
                vec![true, false, true],
                vec![true, false, false],
                vec![false]
            ]
        );
    }
}
