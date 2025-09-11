use pixmux::btree::{BTreeNode, BTreeSpec};

// Helper type alias for testing with unit data
type TestBTree = BTreeNode<(), ()>;

trait TestBTreeExt {
    fn new_leaf() -> Self;
    fn new_branch(first: BTreeNode<(), ()>, second: BTreeNode<(), ()>) -> Self;
    fn new_lastbranch() -> Self;
    fn spec_from(leaf_paths: Vec<Vec<bool>>) -> BTreeSpec;
}

impl TestBTreeExt for TestBTree {
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

    fn spec_from(leaf_paths: Vec<Vec<bool>>) -> BTreeSpec {
        let size = leaf_paths.len();
        BTreeSpec {
            leaf_paths,
            leaf_data: vec![(); size],
            branch_data: vec![(); size - 1],
        }
    }
}

// Use case: Saving to a "state" file
mod collect_spec {
    use super::*;

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
        let spec = tree.get_spec();

        assert_eq!(spec.leaf_paths, expected_paths);
        assert_eq!(spec.leaf_data, expected_leaf_data);
        assert_eq!(spec.branch_data, expected_branch_data);
    }

    // concrete (non-unit) types
    #[test]
    fn test_btree_with_data_on_leaf() {
        let tree = BTreeNode::<i32>::Leaf(42);

        let spec = tree.get_spec();

        assert_eq!(spec.leaf_paths, [[]]);
        assert_eq!(spec.leaf_data, [42]);
        assert_eq!(spec.branch_data, []);
    }

    #[test]
    fn test_btree_with_data_on_branch() {
        let tree = BTreeNode::Branch {
            first: Box::new(BTreeNode::Leaf(())),
            second: Box::new(BTreeNode::Leaf(())),
            data: 42,
        };

        let spec = tree.get_spec();

        assert_eq!(spec.leaf_paths, [[true], [false]]);
        assert_eq!(spec.leaf_data, [(), ()]);
        assert_eq!(spec.branch_data, [42]);
    }
}

// Use case: "Load" from a state file
mod construct_from_spec {
    use super::*;

    #[rstest::rstest]
    #[case(TestBTree::spec_from(vec![vec![]]))] // leaf at root
    #[case(TestBTree::spec_from(vec![vec![true], vec![false]]))] // end-branch at root
    #[case(TestBTree::spec_from(vec![vec![true, true], vec![true, false], vec![false]]))] // first-heavy branching
    #[case(TestBTree::spec_from(vec![vec![true], vec![false, true], vec![false, false]]))] // second-heavy branching
    #[case(TestBTree::spec_from(vec![vec![true, true], vec![true, false], vec![false, true], vec![false, false]]))] // equally deep branching
    fn test_btree(#[case] spec: BTreeSpec) {
        let tree = BTreeNode::<(), ()>::from_spec(&spec).unwrap();

        let coll_spec = tree.get_spec();

        assert_eq!(coll_spec.leaf_paths, spec.leaf_paths);
        assert_eq!(coll_spec.leaf_data, spec.leaf_data);
        assert_eq!(coll_spec.branch_data, spec.branch_data);
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

        let coll_spec = tree.get_spec();

        assert_eq!(coll_spec.leaf_paths, spec.leaf_paths);
        assert_eq!(coll_spec.leaf_data, spec.leaf_data);
        assert_eq!(coll_spec.branch_data, spec.branch_data);
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

// Use case: Splitting a "pane"
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

            let coll_spec = tree.get_spec();

            assert_eq!(coll_spec.leaf_paths, expected_paths);
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
    )] // equally_deep_branching
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

            let coll_spec = tree.get_spec();

            assert_eq!(coll_spec.leaf_data, expected_leaf_data);
            assert_eq!(coll_spec.branch_data, expected_branch_data);
        }
    }
}

// Use case: Splitting a "pane"
mod remove_at {
    use super::*;

    #[rstest::rstest]
    #[case(TestBTree::spec_from(vec![
            vec![]
        ]),
        vec![
            vec![vec![]]
        ]
    )] // leaf at root
    #[case(TestBTree::spec_from(vec![
            vec![true],
            vec![false]
        ]),
        vec![
            vec![vec![]],
            vec![vec![]]
        ]
    )] // branch at root
    #[case(TestBTree::spec_from(vec![
            vec![true, true],
            vec![true, false],
            vec![false]
        ]),
        vec![
            vec![vec![true], vec![false]],
            vec![vec![true], vec![false]],
            vec![vec![true], vec![false]],
        ]
    )] // first-heavy branching
    #[case(TestBTree::spec_from(vec![
            vec![true],
            vec![false, true],
            vec![false, false],
        ]),
        vec![
            vec![vec![true], vec![false]],
            vec![vec![true], vec![false]],
            vec![vec![true], vec![false]],
        ]
    )] // second-heavy branching
    fn test_btree(#[case] spec: BTreeSpec<(), ()>, #[case] post_remove_paths: Vec<Vec<Vec<bool>>>) {
        for (path, expected_paths) in spec.leaf_paths.iter().zip(post_remove_paths) {
            let mut tree = BTreeNode::from_spec(&spec).unwrap();
            tree.remove_leaf_at(path).unwrap();

            let coll_spec = tree.get_spec();

            assert_eq!(coll_spec.leaf_paths, expected_paths);
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
            vec![1]
        ],
        vec![
            vec![]
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
            vec![2],
            vec![1],
        ],
        vec![
            vec![],
            vec![],
        ]
    )] // branch_at_root
    #[case(BTreeSpec {
        leaf_paths: vec![
                vec![true, true],
                vec![true, false],
                vec![false],
            ],
        leaf_data: vec![1, 2, 3],
        branch_data: vec![10, 11],
        },
        vec![
            vec![2,3],
            vec![1,3],
            vec![1,2],
        ],
        vec![
            vec![10],
            vec![10],
            vec![11],
        ]
    )] // first-heavy branching
    #[case(BTreeSpec {
        leaf_paths: vec![
                vec![true],
                vec![false, true],
                vec![false, false],
            ],
        leaf_data: vec![1, 2, 3],
        branch_data: vec![10, 11],
        },
        vec![
            vec![2,3],
            vec![1,3],
            vec![1,2],
        ],
        vec![
            vec![11],
            vec![10],
            vec![10],
        ]
    )] // second-heavy branching
    fn test_btree_with_data(
        #[case] spec: BTreeSpec<i8, i8>,
        #[case] expected_post_remove_leaf_datas: Vec<Vec<i8>>,
        #[case] expected_post_remove_branch_datas: Vec<Vec<i8>>,
    ) {
        for ((path, expected_leaf_data), expected_branch_data) in spec
            .leaf_paths
            .iter()
            .zip(expected_post_remove_leaf_datas)
            .zip(expected_post_remove_branch_datas)
        {
            let mut tree = BTreeNode::from_spec(&spec).unwrap();
            tree.remove_leaf_at(path).unwrap();

            let coll_spec = tree.get_spec();

            assert_eq!(coll_spec.leaf_data, expected_leaf_data);
            assert_eq!(coll_spec.branch_data, expected_branch_data);
        }
    }
}

// Use case: Next/Prev "pane" navigation
mod iter_leaf {
    use super::*;

    #[rstest::rstest]
    #[case(TestBTree::spec_from(vec![
            vec![]
        ]),
        vec![
            vec![]
        ]
    )] // leaf at root
    #[case(TestBTree::spec_from(vec![
            vec![true],
            vec![false],
        ]),
        vec![
            vec![false],
            vec![false],
        ]
    )] // branch at root
    #[case(TestBTree::spec_from(vec![
            vec![true, true],
            vec![true, false],
            vec![false],
        ]),
        vec![
            vec![true, false],
            vec![false],
            vec![false],
        ]
    )] // first-heavy branching
    #[case(TestBTree::spec_from(vec![
            vec![true],
            vec![false, true],
            vec![false, false],
        ]),
        vec![
            vec![false, true],
            vec![false, false],
            vec![false, false],
        ]
    )] // second-heavy branching
    fn test_next(#[case] spec: BTreeSpec<(), ()>, #[case] next_paths: Vec<Vec<bool>>) {
        for (path, next_path) in spec.leaf_paths.iter().zip(next_paths) {
            let tree = BTreeNode::from_spec(&spec).unwrap();

            assert_eq!(tree.get_next_path(path), next_path);
        }
    }

    #[rstest::rstest]
    #[case(TestBTree::spec_from(vec![
            vec![]
        ]),
        vec![
            vec![]
        ]
    )] // leaf at root
    #[case(TestBTree::spec_from(vec![
            vec![true],
            vec![false],
        ]),
        vec![
            vec![true],
            vec![true],
        ]
    )] // branch at root
    #[case(TestBTree::spec_from(vec![
            vec![true, true],
            vec![true, false],
            vec![false],
        ]),
        vec![
            vec![true, true],
            vec![true, true],
            vec![true, false],
        ]
    )] // first-heavy branching
    #[case(TestBTree::spec_from(vec![
            vec![true],
            vec![false, true],
            vec![false, false],
        ]),
        vec![
            vec![true],
            vec![true],
            vec![false, true],
        ]
    )] // second-heavy branching
    fn test_prev(#[case] spec: BTreeSpec<(), ()>, #[case] next_paths: Vec<Vec<bool>>) {
        for (path, next_path) in spec.leaf_paths.iter().zip(next_paths) {
            let tree = BTreeNode::from_spec(&spec).unwrap();

            assert_eq!(tree.get_prev_path(path), next_path);
        }
    }
}
