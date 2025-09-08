use super::*;

// Helper type alias for testing with unit data
type TestBTree = BTreeNode<(), ()>;

mod inspect_tree {
    use super::*;

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

    #[rstest::rstest]
    #[case(TestBTree::new_leaf(),
           vec![vec![]],
           vec![()    ],
           vec![      ],
    )] // leaf at root
    #[case(TestBTree::new_lastbranch(),
           vec![vec![true], vec![false]],
           vec![()        , ()         ],
           vec![         (),            ],
    )] // end-branch at root
    #[case(TestBTree::new_branch(
               TestBTree::new_lastbranch(),
               TestBTree::new_leaf(),
               ),
          vec![vec![true, true], vec![true, false], vec![false]],
          vec![()              , ()               , ()         ],
          vec![               (),                (),           ],
    )] // first-heavy branching
    #[case(TestBTree::new_branch(
               TestBTree::new_leaf(),
               TestBTree::new_lastbranch(),
               ),
           vec![vec![true], vec![false, true], vec![false, false]],
           vec![()        , ()               , ()                ],
           vec![         (),                (),                  ],
    )] // second-heavy branching
    #[case(TestBTree::new_branch(
               TestBTree::new_lastbranch(),
               TestBTree::new_lastbranch(),
               ),
           vec![vec![true, true], vec![true, false], vec![false, true], vec![false, false]],
           vec![()              , ()               , ()               , ()                ],
           vec![               (),                (),                (),                  ],
    )] // equal branching
    fn test_btree(
        #[case] tree: BTreeNode,
        #[case] expected_paths: Vec<Vec<bool>>,
        #[case] expected_leaf_data: Vec<()>,
        #[case] expected_branch_data: Vec<()>,
    ) {
        assert_eq!(tree.collect_paths(), expected_paths);
        assert_eq!(tree.collect_leaf_data(), expected_leaf_data);
        assert_eq!(tree.collect_branch_data(), expected_branch_data);
    }

    // concrete (non-unit) types
    #[test]
    fn test_btree_with_data_on_leaf() {
        let tree = BTreeNode::<i32>::Leaf(42);

        let paths = tree.collect_paths();
        let leaf_data = tree.collect_leaf_data();
        let branch_data = tree.collect_branch_data();

        assert_eq!(paths, [[]]);
        assert_eq!(leaf_data, [42]);
        assert_eq!(branch_data, []);
    }

    #[test]
    fn test_btree_with_data_on_branch() {
        let tree = BTreeNode::Branch {
            first: Box::new(BTreeNode::Leaf(())),
            second: Box::new(BTreeNode::Leaf(())),
            data: 42,
        };

        let paths = tree.collect_paths();
        let leaf_data = tree.collect_leaf_data();
        let branch_data = tree.collect_branch_data();

        assert_eq!(paths, [[true], [false]]);
        assert_eq!(leaf_data, [(), ()]);
        assert_eq!(branch_data, [42]);
    }
}

mod construct_from_spec {
    use super::*;

    impl TestBTree {
        fn spec_from(leaf_paths: Vec<Vec<bool>>) -> BTreeSpec {
            let size = leaf_paths.len();
            BTreeSpec {
                leaf_paths,
                leaf_data: vec![(); size],
                branch_data: vec![(); size - 1],
            }
        }
    }

    #[rstest::rstest]
    #[case(TestBTree::spec_from(vec![vec![]]))] // leaf at root
    #[case(TestBTree::spec_from(vec![vec![true], vec![false]]))] // end-branch at root
    #[case(TestBTree::spec_from(vec![vec![true, true], vec![true, false], vec![false]]))] // first-heavy branching
    #[case(TestBTree::spec_from(vec![vec![true], vec![false, true], vec![false, false]]))] // second-heavy branching
    #[case(TestBTree::spec_from(vec![vec![true, true], vec![true, false], vec![false, true], vec![false, false]]))] // second-heavy branching
    fn test_btree(#[case] spec: BTreeSpec) {
        let tree = BTreeNode::<(), ()>::from_spec(&spec).unwrap();
        assert_eq!(tree.collect_paths(), spec.leaf_paths);
        assert_eq!(tree.collect_leaf_data(), spec.leaf_data);
        assert_eq!(tree.collect_branch_data(), spec.branch_data);
    }

    #[test]
    fn test_btree_with_data() {
        let spec = BTreeSpec {
            leaf_paths: vec![
                vec![true, true],
                vec![true, false],
                vec![false, true],
                vec![false, false],
            ],
            leaf_data: vec![1, 2, 3, 4],
            branch_data: vec![5, 6, 7],
        };
        let tree = BTreeNode::from_spec(&spec).unwrap();
        assert_eq!(tree.collect_paths(), spec.leaf_paths);
        assert_eq!(tree.collect_leaf_data(), spec.leaf_data);
        assert_eq!(tree.collect_branch_data(), spec.branch_data);
    }

    #[rstest::rstest]
    #[case(
        BTreeSpec {
            leaf_paths: vec![],
            leaf_data: vec![],
            branch_data: vec![],
        }, "Prematurely exhausted leaf data"
    )]
    #[case(
        BTreeSpec {
            leaf_paths: vec![],
            leaf_data: vec![(), ()],
            branch_data: vec![],
        }, "Remaining unused leaf data"
    )]
    #[case(
        BTreeSpec {
            leaf_paths: vec![vec![true], vec![false]],
            leaf_data: vec![(),()],
            branch_data: vec![],
        }, "Prematurely exhausted branch data"
    )]
    fn test_btree_fails(#[case] spec: BTreeSpec, #[case] expected_error_msg: String) {
        let result = BTreeNode::from_spec(&spec);
        let actual_error_msg = result.unwrap_err().to_string();
        assert_eq!(actual_error_msg, expected_error_msg);
    }
}
