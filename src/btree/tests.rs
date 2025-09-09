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
        pub fn spec_from(leaf_paths: Vec<Vec<bool>>) -> BTreeSpec {
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
    #[case(TestBTree::spec_from(vec![vec![true, true], vec![true, false], vec![false, true], vec![false, false]]))] // equally deep branching
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
            leaf_paths: vec![vec![]],
            leaf_data: vec![],
            branch_data: vec![],
        }, "Prematurely exhausted leaf data"
    )]
    #[case(
        BTreeSpec {
            leaf_paths: vec![vec![]],
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
    #[case(
        BTreeSpec {
            leaf_paths: vec![vec![true], vec![false]],
            leaf_data: vec![(),()],
            branch_data: vec![(),()],
        }, "Remaining unused branch data"
    )]
    #[case(
        BTreeSpec {
            leaf_paths: vec![vec![true]],
            leaf_data: vec![()],
            branch_data: vec![],
        }, "Non-canonical/invalid path spec"
    )] // only left arm of branch defined
    #[case(
        BTreeSpec {
            leaf_paths: vec![vec![false]],
            leaf_data: vec![()],
            branch_data: vec![],
        }, "Non-canonical/invalid path spec"
    )] // only right arm of branch defined
    #[case(
        BTreeSpec {
            leaf_paths: vec![],
            leaf_data: vec![],
            branch_data: vec![],
        }, "Non-canonical/invalid path spec"
    )] // empty everything
    #[case(
        BTreeSpec {
            leaf_paths: vec![vec![], vec![true], vec![false]],
            leaf_data: vec![(),(),()],
            branch_data: vec![()],
        }, "Non-canonical/invalid path spec"
    )] // branch where a leaf was already specified
    #[case(
        BTreeSpec {
            leaf_paths: vec![vec![true], vec![false], vec![]],
            leaf_data: vec![(),(),()],
            branch_data: vec![()],
        }, "Non-canonical/invalid path spec"
    )] // leaf where a branch was already specified
    fn test_btree_fails(#[case] spec: BTreeSpec, #[case] expected_error_msg: String) {
        let result = BTreeNode::from_spec(&spec);
        let actual_error_msg = result.unwrap_err().to_string();
        assert_eq!(actual_error_msg, expected_error_msg);
    }
}

mod extract_leaf_at {
    use super::*;

    #[rstest::rstest]
    #[case(BTreeSpec {
        leaf_paths: vec![vec![]],
        leaf_data: vec![1],
        branch_data: vec![],
    })] // leaf at root
    #[case(BTreeSpec {
        leaf_paths: vec![vec![true], vec![false]],
        leaf_data: vec![1, 2],
        branch_data: vec![()],
    })] // end-branch at root
    #[case(BTreeSpec {
        leaf_paths: vec![vec![true, true], vec![true, false], vec![false]],
        leaf_data: vec![1, 2, 3],
        branch_data: vec![(), ()],
    })] // first-heavy branching
    #[case(BTreeSpec {
        leaf_paths: vec![vec![true], vec![false, true], vec![false, false]],
        leaf_data: vec![1, 2, 3],
        branch_data: vec![(), ()],
    })] // second-heavy branching
    #[case(BTreeSpec {
        leaf_paths: vec![vec![true, true], vec![true, false], vec![false, true], vec![false, false]],
        leaf_data: vec![1, 2, 3, 4],
        branch_data: vec![(), (), ()],
    })] // equally deep branching
    fn test_btree(#[case] spec: BTreeSpec<i8, ()>) {
        let tree = BTreeNode::from_spec(&spec).unwrap();
        for (path, expected_data) in spec.leaf_paths.iter().zip(spec.leaf_data) {
            let leaf = tree.get_leaf_at(path).unwrap();
            if let BTreeNode::Leaf(actual_data) = *leaf {
                assert_eq!(actual_data, expected_data);
            }
        }
    }
}

mod split_at {
    use super::*;

    #[rstest::rstest]
    #[case(TestBTree::spec_from(vec![
            vec![]
        ]),
        vec![
            vec![vec![true], vec![false]]
        ]
    )] // leaf at root
    #[case(TestBTree::spec_from(vec![
            vec![true],
            vec![false]
        ]),
        vec![
            vec![vec![true, true], vec![true, false], vec![false]],
            vec![vec![true], vec![false, true], vec![false, false]]
        ]
    )] // end-branch at root
    #[case(TestBTree::spec_from(vec![
            vec![true, true],
            vec![true, false],
            vec![false],
        ]),
        vec![
            vec![
                vec![true, true, true],
                vec![true, true, false],
                vec![true, false],
                vec![false],
            ],
            vec![
                vec![true, true],
                vec![true, false, true],
                vec![true, false, false],
                vec![false],
            ],
            vec![
                vec![true, true],
                vec![true, false],
                vec![false, true],
                vec![false, false],
            ],
        ]
    )] // first-heavy branching
    #[case(TestBTree::spec_from(vec![
            vec![true],
            vec![false, true],
            vec![false, false]
        ]),
        vec![
            vec![
                vec![true, true],
                vec![true, false],
                vec![false, true],
                vec![false, false],
            ],
            vec![
                vec![true],
                vec![false, true, true],
                vec![false, true, false],
                vec![false, false],
            ],
            vec![
                vec![true],
                vec![false, true],
                vec![false, false, true],
                vec![false, false, false],
            ],
        ]
    )] // second-heavy branching
    fn test_btree(#[case] spec: BTreeSpec<(), ()>, #[case] split_paths: Vec<Vec<Vec<bool>>>) {
        for (mut path, expected_paths) in spec.leaf_paths.iter().zip(split_paths) {
            let mut tree = BTreeNode::from_spec(&spec).unwrap();
            tree.split_leaf_at(&mut path, ()).unwrap();
            assert_eq!(tree.collect_paths(), expected_paths);
        }
    }

    #[rstest::rstest]
    #[case(BTreeSpec {
        leaf_paths: vec![
                vec![]
            ],
        leaf_data: vec![1],
        branch_data: vec![],
        },
        vec![
            vec![1, 1]
        ],
        vec![
            vec![5]
        ]
    )] // leaf at root
    #[case(BTreeSpec {
        leaf_paths: vec![
                vec![true],
                vec![false],
            ],
        leaf_data: vec![1, 2],
        branch_data: vec![10],
        },
        vec![
            vec![1, 1, 2],
            vec![1, 2, 2],
        ],
        vec![
            vec![10, 5],
            vec![10, 5],
        ]
    )] // branch_at_root
    #[case(BTreeSpec {
        leaf_paths: vec![
                vec![true, true],
                vec![true, false],
                vec![false, true],
                vec![false, false],
            ],
        leaf_data: vec![1, 2, 3, 4],
        branch_data: vec![10, 11, 12],
        },
        vec![
            vec![1, 1, 2, 3, 4],
            vec![1, 2, 2, 3, 4],
            vec![1, 2, 3, 3, 4],
            vec![1, 2, 3, 4, 4],
        ],
        vec![
            vec![10, 11, 5, 12],
            vec![10, 11, 5, 12],
            vec![10, 11, 12, 5],
            vec![10, 11, 12, 5],
        ]
    )] // branch_at_root
    fn test_btree_with_data(
        #[case] spec: BTreeSpec<i8, i8>,
        #[case] expected_post_split_leaf_datas: Vec<Vec<i8>>,
        #[case] expected_post_split_branch_datas: Vec<Vec<i8>>,
    ) {
        for ((mut path, expected_leaf_data), expected_branch_data) in spec
            .leaf_paths
            .iter()
            .zip(expected_post_split_leaf_datas)
            .zip(expected_post_split_branch_datas)
        {
            let mut tree = BTreeNode::from_spec(&spec).unwrap();
            tree.split_leaf_at(&mut path, 5).unwrap();
            assert_eq!(tree.collect_leaf_data(), expected_leaf_data);
            assert_eq!(tree.collect_branch_data(), expected_branch_data);
        }
    }
}
