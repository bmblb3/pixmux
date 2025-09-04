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

    pub fn collect_leaf_paths(&self) -> Vec<Vec<bool>> {
        vec![vec![true], vec![false]]
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
}
