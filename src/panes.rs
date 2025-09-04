use color_eyre::eyre;
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

macro_rules! impl_get_node_at {
    ($self:ident, $path:ident, $method:ident) => {
        if $path.is_empty() {
            Ok($self)
        } else {
            match $self {
                Pane::Leaf { .. } => Err(eyre::eyre!("Path leads beyond a leaf node")),
                Pane::Split { first, second, .. } => {
                    let child = if $path[0] { first } else { second };
                    child.$method(&$path[1..])
                }
            }
        }
    };
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

    pub fn get_node_at_mut(&mut self, path: &[bool]) -> eyre::Result<&mut Pane> {
        impl_get_node_at!(self, path, get_node_at_mut)
    }

    pub fn get_node_at(&self, path: &[bool]) -> eyre::Result<&Pane> {
        impl_get_node_at!(self, path, get_node_at)
    }

    pub fn split_leaf(&mut self, path: &[bool], direction: layout::Direction) -> eyre::Result<()> {
        let pane = self.get_node_at_mut(path)?;
        match pane {
            Pane::Leaf { .. } => {
                *pane = Self::new_split(direction);
                Ok(())
            }
            Pane::Split { .. } => Err(eyre::eyre!("Can only split a leaf node")),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Test creating fresh nodes
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
                matches!(tree.get_node_at(&[]).unwrap(), &Pane::Split { direction: d, .. } if d==direction)
            );
            assert!(matches!(
                tree.get_node_at(&[true]).unwrap(),
                &Pane::Leaf { .. }
            ));
            assert!(matches!(
                tree.get_node_at(&[false]).unwrap(),
                &Pane::Leaf { .. }
            ));
        }
    }

    // Test for leaves collection
    #[test]
    fn test_new_split_collect_leaves() {
        let tree = Pane::new_split(layout::Direction::Horizontal);
        let paths = tree.collect_leaf_paths();

        assert_eq!(paths, vec![vec![true], vec![false]]);
    }

    #[test]
    fn test_a1a2b_split_collect_leaves() {
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
    fn test_ab1b2_split_collect_leaves() {
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
    fn test_deep_nesting_path_first_heavy_collect_leaves() {
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
    fn test_deep_nesting_path_second_heavy_collect_leaves() {
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

    // Test for some valid splits
    #[test]
    fn test_split_root_leaf() {
        let directions = vec![layout::Direction::Horizontal, layout::Direction::Vertical];

        for direction in directions {
            let mut tree = Pane::new_leaf();
            tree.split_leaf(&[], direction).unwrap();

            let paths = tree.collect_leaf_paths();
            assert_eq!(paths, vec![vec![true], vec![false]]);
            assert!(matches!(
                tree.get_node_at(&[]).unwrap(),
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
            tree.split_leaf(&[true], direction).unwrap();

            let paths = tree.collect_leaf_paths();
            assert_eq!(
                paths,
                vec![vec![true, true], vec![true, false], vec![false]]
            );
            assert!(matches!(
                tree.get_node_at(&[true]).unwrap(),
                &Pane::Split {
                    direction: d,
                    ..
                } if d==direction
            ));
        }
    }

    // Getting a node from a path can fail if the path is "invalid"
    //   i.e the path charts a route BEYOND a leaf
    #[test]
    fn test_err_getting_a_path_beyond_a_leaf() {
        let tree = Pane::new_split(layout::Direction::Horizontal);
        let result = tree.get_node_at(&[true, true]);
        assert!(result.is_err());
    }

    // Splits can fail for two reasons
    #[test]
    fn test_err_splitting_a_split() {
        let mut tree = Pane::new_split(layout::Direction::Horizontal);
        let result = tree.split_leaf(&[], layout::Direction::Horizontal);
        assert!(result.is_err());
    }

    #[test]
    fn test_err_splitting_a_path_beyond_a_leaf() {
        let mut tree = Pane::new_split(layout::Direction::Horizontal);
        let result = tree.split_leaf(&[true, true], layout::Direction::Horizontal);
        assert!(result.is_err());
    }
}
