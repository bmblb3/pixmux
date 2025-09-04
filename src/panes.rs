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

    pub fn get_node_at(&self, path: &[bool]) -> &Pane {
        if path.is_empty() {
            return self;
        }
        match self {
            Pane::Split { first, second, .. } => {
                let go_first = path[0];
                if go_first {
                    first.get_node_at(&path[1..])
                } else {
                    second.get_node_at(&path[1..])
                }
            }
            Pane::Leaf { .. } => self,
        }
    }

    pub fn split_leaf(&mut self, path: &[bool], direction: layout::Direction) -> bool {
        match self {
            Pane::Split { first, second, .. } => {
                let go_first = path[0];
                if go_first {
                    first.split_leaf(&path[1..], direction)
                } else {
                    second.split_leaf(&path[1..], direction)
                }
            }
            Pane::Leaf { .. } => {
                *self = Self::new_split(direction);
                true
            }
        }
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
    fn test_split_directions() {
        let directions = vec![layout::Direction::Horizontal, layout::Direction::Vertical];

        for direction in directions {
            let tree = Pane::new_split(direction);

            assert!(
                matches!(tree.get_node_at(&[]), &Pane::Split { direction: d, .. } if d==direction)
            );
            assert!(matches!(tree.get_node_at(&[true]), &Pane::Leaf { .. }));
            assert!(matches!(tree.get_node_at(&[false]), &Pane::Leaf { .. }));
        }
    }

    #[test]
    fn test_new_split_get_leaves() {
        let tree = Pane::new_split(layout::Direction::Horizontal);
        let paths = tree.collect_leaf_paths();

        assert_eq!(paths, vec![vec![true], vec![false]]);
    }

    #[test]
    fn test_a1a2b_split_get_leaves() {
        let tree = Pane::Split {
            direction: layout::Direction::Vertical,
            pct: 50,
            first: Box::new(Pane::new_split(layout::Direction::Horizontal)),
            second: Box::new(Pane::new_leaf()),
        };

        let paths = tree.collect_leaf_paths();

        assert_eq!(
            paths,
            vec![vec![true, true], vec![true, false], vec![false]]
        );
    }

    #[test]
    fn test_ab1b2_split_get_leaves() {
        let tree = Pane::Split {
            direction: layout::Direction::Vertical,
            pct: 50,
            first: Box::new(Pane::new_leaf()),
            second: Box::new(Pane::new_split(layout::Direction::Horizontal)),
        };

        let paths = tree.collect_leaf_paths();

        assert_eq!(
            paths,
            vec![vec![true], vec![false, true], vec![false, false]]
        );
    }

    #[test]
    fn test_deep_nesting_path_first_heavy() {
        let tree = Pane::Split {
            direction: layout::Direction::Horizontal,
            pct: 50,
            first: Box::new(Pane::Split {
                direction: layout::Direction::Vertical,
                pct: 50,
                first: Box::new(Pane::new_split(layout::Direction::Horizontal)),
                second: Box::new(Pane::new_leaf()),
            }),
            second: Box::new(Pane::new_leaf()),
        };

        let paths = tree.collect_leaf_paths();

        assert_eq!(
            paths,
            vec![
                vec![true, true, true],
                vec![true, true, false],
                vec![true, false],
                vec![false]
            ]
        );
    }

    #[test]
    fn test_deep_nesting_path_second_heavy() {
        let tree = Pane::Split {
            direction: layout::Direction::Horizontal,
            pct: 50,
            first: Box::new(Pane::new_leaf()),
            second: Box::new(Pane::Split {
                direction: layout::Direction::Vertical,
                pct: 50,
                first: Box::new(Pane::new_leaf()),
                second: Box::new(Pane::new_split(layout::Direction::Horizontal)),
            }),
        };

        let paths = tree.collect_leaf_paths();

        assert_eq!(
            paths,
            vec![
                vec![true],
                vec![false, true],
                vec![false, false, true],
                vec![false, false, false]
            ]
        );
    }

    #[test]
    fn test_split_leaf_root() {
        let directions = vec![layout::Direction::Horizontal, layout::Direction::Vertical];

        for direction in directions {
            let mut tree = Pane::new_leaf();
            let success = tree.split_leaf(&[], direction);

            assert!(success);
            let paths = tree.collect_leaf_paths();
            assert_eq!(paths, vec![vec![true], vec![false]]);
            assert!(matches!(
                tree.get_node_at(&[]),
                &Pane::Split {
                    direction: d,
                    ..
                } if d==direction
            ));
        }
    }

    #[test]
    fn test_split_leaf_nested() {
        let directions = vec![layout::Direction::Horizontal, layout::Direction::Vertical];

        for direction in directions {
            let mut tree = Pane::new_split(layout::Direction::Horizontal);
            let success = tree.split_leaf(&[true], direction);

            assert!(success);
            let paths = tree.collect_leaf_paths();
            assert_eq!(
                paths,
                vec![vec![true, true], vec![true, false], vec![false]]
            );
            assert!(matches!(
                tree.get_node_at(&[true]),
                &Pane::Split {
                    direction: d,
                    ..
                } if d==direction
            ));
        }
    }
}
