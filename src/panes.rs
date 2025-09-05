use color_eyre::eyre::{self, Ok, OptionExt};
use ratatui::layout;

use crate::{AdjustDirection, cycle_index};

#[derive(Clone)]
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

impl Default for Pane {
    fn default() -> Self {
        Pane::new_leaf()
    }
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
    #![warn(clippy::used_underscore_binding)]

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

    pub fn split_leaf_at(
        &mut self,
        path: &[bool],
        direction: layout::Direction,
    ) -> eyre::Result<Vec<bool>> {
        let pane = self.get_node_at_mut(path)?;
        match pane {
            Pane::Leaf { .. } => {
                *pane = Self::new_split(direction);
                let mut result = path.to_vec();
                result.push(true);
                Ok(result)
            }
            Pane::Split { .. } => Err(eyre::eyre!("Can only split a leaf node")),
        }
    }

    pub fn remove_leaf_at(&mut self, path: &[bool]) -> eyre::Result<Vec<bool>> {
        let to_remove = self.get_node_at(path)?;
        match to_remove {
            Pane::Leaf { .. } => {}
            Pane::Split { .. } => {
                return Err(eyre::eyre!("Not allowed to remove a split node"));
            }
        }

        let mut parent_path = path.to_vec();
        let removing_first_child = match parent_path.pop() {
            Some(boolval) => boolval,
            _ => {
                let root = self.get_node_at_mut(path)?;
                *root = Pane::new_leaf();
                return Ok(vec![]);
            }
        };

        let parent = self.get_node_at_mut(&parent_path)?;

        match parent {
            Pane::Leaf { .. } => {
                return Err(eyre::eyre!("Parent does not seem to be a split node!"));
            }
            Pane::Split { first, second, .. } => {
                let sibling = if removing_first_child {
                    std::mem::take(second)
                } else {
                    std::mem::take(first)
                };
                *parent = *sibling;
            }
        };
        Ok(parent_path)
    }

    pub fn cycle(&self, path: &[bool], direction: AdjustDirection) -> eyre::Result<Vec<bool>> {
        let leaf_paths = self.collect_leaf_paths();
        let current_pos = leaf_paths
            .iter()
            .position(|x| x == path)
            .ok_or_eyre("Could not find current path in leaf paths")?;
        let cycled_index = cycle_index(current_pos, leaf_paths.len(), direction);
        let cycled_path = leaf_paths
            .get(cycled_index)
            .ok_or_eyre("Could not find cycled path")?;
        Ok(cycled_path.clone())
    }

    pub fn resize_leaf_at(
        &mut self,
        path: &[bool],
        direction: layout::Direction,
        delta: i8,
    ) -> eyre::Result<()> {
        if path.is_empty() {
            return Ok(());
        }
        let (parent_path, _) = match path.split_at_checked(path.len().saturating_sub(1)) {
            Some(val) => val,
            _ => {
                return Ok(());
            }
        };
        let parent_node = self.get_node_at_mut(parent_path)?;
        if let Pane::Split {
            direction: splitdir,
            pct,
            ..
        } = parent_node
        {
            if *splitdir == direction {
                *pct = (*pct as i8 + delta).clamp(5, 95) as u8;
            } else {
                self.resize_leaf_at(parent_path, direction, delta)?;
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::AdjustDirection;

    // Test creating fresh nodes
    #[test]
    fn test_new_leaf_root() {
        let tree = Pane::new_leaf();

        assert!(matches!(tree, Pane::Leaf { .. }));
    }

    #[test]
    fn test_new_split_root() {
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
    fn test_get_node_at() {
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
    fn test_new_root_split_collect_leaves() {
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
            let first_created_path = tree.split_leaf_at(&[], direction).unwrap();

            let paths = tree.collect_leaf_paths();
            assert_eq!(paths, vec![vec![true], vec![false]]);
            assert_eq!(first_created_path, vec![true]);
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
            let first_created_path = tree.split_leaf_at(&[true], direction).unwrap();

            let paths = tree.collect_leaf_paths();
            assert_eq!(
                paths,
                vec![vec![true, true], vec![true, false], vec![false]]
            );
            assert_eq!(first_created_path, vec![true, true]);
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
        let result = tree.split_leaf_at(&[], layout::Direction::Horizontal);
        assert!(result.is_err());
    }

    #[test]
    fn test_err_splitting_a_path_beyond_a_leaf() {
        let mut tree = Pane::new_split(layout::Direction::Horizontal);
        let result = tree.split_leaf_at(&[true, true], layout::Direction::Horizontal);
        assert!(result.is_err());
    }

    // Remove nodes
    #[test]
    fn test_remove_a_simple_leaf_node() {
        let test_cases = [(true, 2), (false, 1)];

        for test_case in test_cases {
            let (remove_child, expected) = test_case;

            let mut tree = Pane::Split {
                direction: layout::Direction::Vertical,
                pct: 50,
                first: Box::new(Pane::Leaf { image_id: 1 }),
                second: Box::new(Pane::Leaf { image_id: 2 }),
            };
            let promoted_sibling_path = tree.remove_leaf_at(&[remove_child]).unwrap();

            assert_eq!(promoted_sibling_path, vec![]);
            assert!(matches!(tree, Pane::Leaf { image_id: e } if e==expected));
        }
    }

    #[test]
    fn test_remove_leaf_nested() {
        let mut tree = Pane::Split {
            direction: layout::Direction::Vertical,
            pct: 50,
            first: Box::new(Pane::new_split(layout::Direction::Horizontal)),
            second: Box::new(Pane::new_leaf()),
        };

        let promoted_sibling_path = tree.remove_leaf_at(&[true, false]).unwrap();
        let paths = tree.collect_leaf_paths();

        assert_eq!(promoted_sibling_path, vec![true]);
        assert_eq!(paths, vec![vec![true], vec![false]]);
    }

    #[test]
    fn test_remove_root_node() {
        let mut tree = Pane::Leaf { image_id: 1 };
        let promoted_sibling_path = tree.remove_leaf_at(&[]).unwrap();

        assert_eq!(promoted_sibling_path, vec![]);
        assert!(matches!(tree, Pane::Leaf { image_id: 0 }));
    }

    // Removal fails because
    // 1. Removing a split (business logic disallows this)
    // 2. Removing beyond a node
    #[test]
    fn test_remove_split() {
        let mut tree = Pane::new_split(layout::Direction::Horizontal);
        let result = tree.remove_leaf_at(&[]);
        assert!(result.is_err());
    }

    #[test]
    fn test_remove_beyond_a_node() {
        let mut tree = Pane::new_split(layout::Direction::Horizontal);
        let result = tree.remove_leaf_at(&[true, true]);
        assert!(result.is_err());
    }

    // Cycle leafs
    #[test]
    fn test_cycle_root_leaf() {
        let tree = Pane::new_leaf();

        let next = tree.cycle(&[], AdjustDirection::Forward).unwrap();
        assert_eq!(next, vec![]);

        let prev = tree.cycle(&[], AdjustDirection::Backward).unwrap();
        assert_eq!(prev, vec![]);
    }

    #[test]
    fn test_cycle_nested_leaf() {
        let tree = Pane::new_split(layout::Direction::Horizontal);

        let next = tree.cycle(&[true], AdjustDirection::Forward).unwrap();
        assert_eq!(next, vec![false]);

        let next = tree.cycle(&[false], AdjustDirection::Forward).unwrap();
        assert_eq!(next, vec![true]);

        let prev = tree.cycle(&[true], AdjustDirection::Backward).unwrap();
        assert_eq!(prev, vec![false]);

        let prev = tree.cycle(&[false], AdjustDirection::Backward).unwrap();
        assert_eq!(prev, vec![true]);
    }

    #[test]
    fn test_cycle_deeply_nested_leaf() {
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

        assert_eq!(
            tree.cycle(&[true], AdjustDirection::Forward).unwrap(),
            vec![false, true]
        );

        assert_eq!(
            tree.cycle(&[false, false, false], AdjustDirection::Forward)
                .unwrap(),
            vec![true]
        );

        assert_eq!(
            tree.cycle(&[true], AdjustDirection::Backward).unwrap(),
            vec![false, false, false]
        );

        assert_eq!(
            tree.cycle(&[false, false, false], AdjustDirection::Backward)
                .unwrap(),
            vec![false, false, true]
        );
    }

    // Cycle fails when invalid path is provided
    #[test]
    fn test_cycle_invalid_leaf() {
        let tree = Pane::new_leaf();

        let result = tree.cycle(&[true], AdjustDirection::Forward);
        assert!(result.is_err());
    }

    // Resize
    #[test]
    fn test_redundant_resize_root_leaf() {
        let mut tree = Pane::new_leaf();

        tree.resize_leaf_at(&[], layout::Direction::Vertical, 5)
            .unwrap();

        assert!(matches!(tree, Pane::Leaf { .. }));
    }

    #[test]
    fn test_redundant_vresize_leaf_under_horizontally_stacked_split_root() {
        let mut tree = Pane::new_split(layout::Direction::Horizontal);
        tree.resize_leaf_at(&[true], layout::Direction::Vertical, 10)
            .unwrap();
        assert!(matches!(
            tree.get_node_at(&[]).unwrap(),
            Pane::Split {
                direction: layout::Direction::Horizontal,
                pct: 50,
                ..
            }
        ))
    }

    #[test]
    fn test_hresize_leaf_under_horizontally_stacked_split_root() {
        let mut tree = Pane::new_split(layout::Direction::Horizontal);
        tree.resize_leaf_at(&[true], layout::Direction::Horizontal, 10)
            .unwrap();
        assert!(matches!(
            tree.get_node_at(&[]).unwrap(),
            Pane::Split {
                direction: layout::Direction::Horizontal,
                pct: 60,
                ..
            }
        ))
    }

    #[test]
    fn test_hresize_leaf_under_hv_split_root() {
        let mut tree = Pane::Split {
            direction: layout::Direction::Horizontal,
            pct: 50,
            first: Box::new(Pane::Split {
                direction: layout::Direction::Vertical,
                pct: 50,
                first: Box::new(Pane::new_leaf()),
                second: Box::new(Pane::new_leaf()),
            }),
            second: Box::new(Pane::new_leaf()),
        };

        tree.resize_leaf_at(&[true, true], layout::Direction::Horizontal, 10)
            .unwrap();

        assert!(matches!(
            tree.get_node_at(&[true]).unwrap(),
            Pane::Split {
                direction: layout::Direction::Vertical,
                pct: 50, // this is a vsplit so should not be resized
                ..
            }
        ));
        assert!(matches!(
            tree.get_node_at(&[]).unwrap(),
            Pane::Split {
                direction: layout::Direction::Horizontal,
                pct: 60, // this is the nearest parent hsplit so we resize this
                ..
            }
        ));
    }

    #[test]
    fn test_hresize_leaf_under_hh_split_root() {
        let mut tree = Pane::Split {
            direction: layout::Direction::Horizontal,
            pct: 50,
            first: Box::new(Pane::Split {
                direction: layout::Direction::Horizontal,
                pct: 50,
                first: Box::new(Pane::new_leaf()),
                second: Box::new(Pane::new_leaf()),
            }),
            second: Box::new(Pane::new_leaf()),
        };

        tree.resize_leaf_at(&[true, true], layout::Direction::Horizontal, 10)
            .unwrap();

        assert!(matches!(
            tree.get_node_at(&[true]).unwrap(),
            Pane::Split {
                direction: layout::Direction::Horizontal,
                pct: 60, // this is the nearest parent hsplit so we resize this
                ..
            }
        ));
        assert!(matches!(
            tree.get_node_at(&[]).unwrap(),
            Pane::Split {
                direction: layout::Direction::Horizontal,
                pct: 50, // a child split was already resized, so we keep this
                ..
            }
        ));
    }
}
