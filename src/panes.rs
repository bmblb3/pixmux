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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_leaf() {
        let tree = Pane::new_leaf();

        assert!(matches!(tree, Pane::Leaf { .. }));
    }
}
