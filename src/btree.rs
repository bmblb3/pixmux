pub enum BTreeNode<L = (), B = ()> {
    Leaf(L),
    Branch {
        first: Box<BTreeNode<L, B>>,
        second: Box<BTreeNode<L, B>>,
        data: B,
    },
}

impl<L> BTreeNode<L> {
    pub fn get_paths(&self) -> Vec<Vec<bool>> {
        match self {
            BTreeNode::Leaf(_) => vec![vec![]],
            BTreeNode::Branch { .. } => vec![vec![true], vec![false]],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_btree_with_leaf_at_root_returns_computed_paths() {
        let tree = BTreeNode::Leaf(());

        let paths = tree.get_paths();

        assert_eq!(paths, [[]]);
    }

    #[test]
    fn test_btree_with_leaf_at_root_with_data_returns_computed_paths() {
        let tree = BTreeNode::Leaf(42);

        let paths = tree.get_paths();

        assert_eq!(paths, [[]]);
    }

    #[test]
    fn test_btree_with_simple_branch_at_root_returns_computed_paths() {
        let tree = BTreeNode::Branch {
            first: Box::new(BTreeNode::Leaf(())),
            second: Box::new(BTreeNode::Leaf(())),
            data: (),
        };

        let paths = tree.get_paths();

        assert_eq!(paths, [[true], [false]]);
    }
}
