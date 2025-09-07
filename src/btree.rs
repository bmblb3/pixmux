pub enum BTreeNode<L = (), B = ()> {
    Leaf(L),
    Branch {
        first: Box<BTreeNode<L, B>>,
        second: Box<BTreeNode<L, B>>,
        data: B,
    },
}

impl<L, B> BTreeNode<L, B> {
    pub fn collect_paths(&self) -> Vec<Vec<bool>> {
        let mut all_paths = Vec::new();
        Self::collect_paths_impl(self, &mut Vec::new(), &mut all_paths);
        all_paths
    }

    fn collect_paths_impl(
        node: &Self,
        current_path: &mut Vec<bool>,
        all_paths: &mut Vec<Vec<bool>>,
    ) {
        match node {
            Self::Leaf(_) => all_paths.push(current_path.to_vec()),
            Self::Branch { first, second, .. } => {
                for (child, bool) in [(first, true), (second, false)] {
                    current_path.push(bool);
                    Self::collect_paths_impl(child, current_path, all_paths);
                    current_path.pop();
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    type TestBTree = BTreeNode<(), ()>;

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
    fn test_btree_returns_computed_paths() {
        let test_cases = [
            (
                // leaf at root
                TestBTree::Leaf(()),
                vec![vec![]],
            ),
            (
                // simple branch at root
                TestBTree::Branch {
                    first: Box::new(BTreeNode::Leaf(())),
                    second: Box::new(BTreeNode::Leaf(())),
                    data: (),
                },
                vec![vec![true], vec![false]],
            ),
            (
                // first heavy tree
                TestBTree::new_branch(
                    TestBTree::new_branch(TestBTree::new_leaf(), TestBTree::new_leaf()),
                    TestBTree::new_leaf(),
                ),
                vec![vec![true, true], vec![true, false], vec![false]],
            ),
            (
                // second heavy tree
                TestBTree::new_branch(
                    TestBTree::new_leaf(),
                    TestBTree::new_branch(TestBTree::new_leaf(), TestBTree::new_leaf()),
                ),
                vec![vec![true], vec![false, true], vec![false, false]],
            ),
            (
                // two splits at root
                TestBTree::new_branch(
                    TestBTree::new_branch(TestBTree::new_leaf(), TestBTree::new_leaf()),
                    TestBTree::new_branch(TestBTree::new_leaf(), TestBTree::new_leaf()),
                ),
                vec![
                    vec![true, true],
                    vec![true, false],
                    vec![false, true],
                    vec![false, false],
                ],
            ),
        ];

        for (tree, expected) in test_cases {
            let paths = tree.collect_paths();
            assert_eq!(paths, expected);
        }
    }

    #[test]
    fn test_btree_with_leaf_at_root_with_data_returns_computed_paths() {
        let tree = BTreeNode::<i32>::Leaf(42);

        let paths = tree.collect_paths();

        assert_eq!(paths, [[]]);
    }

    #[test]
    fn test_btree_with_simple_branch_with_data_at_root_returns_computed_paths() {
        let tree = BTreeNode::Branch {
            first: Box::new(BTreeNode::Leaf(())),
            second: Box::new(BTreeNode::Leaf(())),
            data: 42,
        };

        let paths = tree.collect_paths();

        assert_eq!(paths, [[true], [false]]);
    }
}
