use super::*;

// Helper type alias for testing with unit data
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

    fn new_lastbranch() -> Self {
        BTreeNode::Branch {
            first: Box::new(Self::new_leaf()),
            second: Box::new(Self::new_leaf()),
            data: (),
        }
    }
}

mod path_collection {
    use super::*;

    #[rstest::rstest]
    #[case(TestBTree::new_leaf(),       vec![vec![]])] // leaf at root
    #[case(TestBTree::new_lastbranch(), vec![vec![true], vec![false]])] // end-branch at root
    #[case(TestBTree::new_branch(
                          TestBTree::new_lastbranch(),
                          TestBTree::new_leaf(),
                      ),                vec![vec![true, true], vec![true, false], vec![false]])] // first-heavy branching
    #[case(TestBTree::new_branch(
                          TestBTree::new_leaf(),
                          TestBTree::new_lastbranch(),
                      ),                vec![vec![true], vec![false, true], vec![false, false]])] // second-heavy branching
    #[case(TestBTree::new_branch(
                          TestBTree::new_lastbranch(),
                          TestBTree::new_lastbranch(),
                      ),                vec![
                                            vec![true, true],
                                            vec![true, false],
                                            vec![false, true],
                                            vec![false, false],
                                        ])] // equal branching
    fn test_btree_returns_computed_paths_parametric(
        #[case] tree: BTreeNode,
        #[case] expected: Vec<Vec<bool>>,
    ) {
        assert_eq!(tree.collect_paths(), expected);
    }

    // concrete (non-unit) types
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
