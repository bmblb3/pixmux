pub enum BTreeNode<L = (), B = ()> {
    Leaf(L),
    Branch {
        first: Box<BTreeNode<L, B>>,
        second: Box<BTreeNode<L, B>>,
        data: B,
    },
}

impl<L, B> BTreeNode<L, B> {
    pub fn get_paths(&self) -> Vec<Vec<bool>> {
        match self {
            BTreeNode::Leaf(_) => vec![vec![]],
            BTreeNode::Branch { .. } => {
                let mut all_paths = Vec::new();
                let mut current_path = Vec::new();
                Self::get_paths_impl(self, &mut current_path, &mut all_paths);
                all_paths
            }
        }
    }

    fn get_paths_impl(node: &Self, current_path: &mut Vec<bool>, all_paths: &mut Vec<Vec<bool>>) {
        match node {
            Self::Leaf(_) => {
                all_paths.push(current_path.to_vec());
            }
            Self::Branch { first, second, .. } => {
                current_path.push(true);
                Self::get_paths_impl(first, current_path, all_paths);
                current_path.pop();

                current_path.push(false);
                Self::get_paths_impl(second, current_path, all_paths);
                current_path.pop();
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    type TestBTree = BTreeNode<(), ()>;

    #[test]
    fn test_btree_with_leaf_at_root_returns_computed_paths() {
        let tree = TestBTree::Leaf(());

        let paths = tree.get_paths();

        assert_eq!(paths, [[]]);
    }

    #[test]
    fn test_btree_with_leaf_at_root_with_data_returns_computed_paths() {
        type TestConcreteBTree = BTreeNode<i8, ()>;
        let tree = TestConcreteBTree::Leaf(42);

        let paths = tree.get_paths();

        assert_eq!(paths, [[]]);
    }

    #[test]
    fn test_btree_with_simple_branch_at_root_returns_computed_paths() {
        let tree = TestBTree::Branch {
            first: Box::new(BTreeNode::Leaf(())),
            second: Box::new(BTreeNode::Leaf(())),
            data: (),
        };

        let paths = tree.get_paths();

        assert_eq!(paths, [[true], [false]]);
    }

    #[test]
    fn test_btree_with_simple_branch_with_data_at_root_returns_computed_paths() {
        type TestConcreteBTree = BTreeNode<(), i8>;
        let tree = TestConcreteBTree::Branch {
            first: Box::new(BTreeNode::Leaf(())),
            second: Box::new(BTreeNode::Leaf(())),
            data: 42,
        };

        let paths = tree.get_paths();

        assert_eq!(paths, [[true], [false]]);
    }

    impl TestBTree {
        fn new_leaf() -> Self {
            BTreeNode::Leaf(())
        }

        fn new_branch(first: BTreeNode<(), ()>, second: BTreeNode<(), ()>) -> Self {
            BTreeNode::Branch {
                first: Box::new(first),
                second: Box::new(second),
                data: (),
            }
        }
    }

    #[test]
    fn test_btree_with_first_heavy_branch_returns_computed_paths() {
        let tree = TestBTree::new_branch(
            TestBTree::new_branch(TestBTree::new_leaf(), TestBTree::new_leaf()),
            TestBTree::new_leaf(),
        );

        let paths = tree.get_paths();

        assert_eq!(paths, [vec![true, true], vec![true, false], vec![false]]);
    }

    #[test]
    fn test_btree_with_second_heavy_branch_returns_computed_paths() {
        let tree = TestBTree::new_branch(
            TestBTree::new_leaf(),
            TestBTree::new_branch(TestBTree::new_leaf(), TestBTree::new_leaf()),
        );

        let paths = tree.get_paths();

        assert_eq!(paths, [vec![true], vec![false, true], vec![false, false]]);
    }
}
