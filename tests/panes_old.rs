use pixmux::{AdjustDirection, Pane};
use ratatui::layout;

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
        let second_created_path = tree.split_leaf_at(&[], direction).unwrap();

        let paths = tree.collect_leaf_paths();
        assert_eq!(paths, vec![vec![true], vec![false]]);
        assert_eq!(second_created_path, vec![false]);
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
        let second_created_path = tree.split_leaf_at(&[true], direction).unwrap();

        let paths = tree.collect_leaf_paths();
        assert_eq!(
            paths,
            vec![vec![true, true], vec![true, false], vec![false]]
        );
        assert_eq!(second_created_path, vec![true, false]);
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

#[test]
fn test_remove_leaf_sibling_of_a_split() {
    let mut tree = Pane::Split {
        direction: layout::Direction::Vertical,
        pct: 50,
        first: Box::new(Pane::new_leaf()),
        second: Box::new(Pane::new_split(layout::Direction::Horizontal)),
    };

    let promoted_sibling_path = tree.remove_leaf_at(&[true]).unwrap();
    assert_eq!(promoted_sibling_path, vec![true]);
    assert!(matches!(tree, Pane::Split { .. }));
}

#[test]
fn test_remove_leaf_sibling_of_a_split_other_side() {
    let mut tree = Pane::Split {
        direction: layout::Direction::Vertical,
        pct: 50,
        first: Box::new(Pane::new_split(layout::Direction::Horizontal)),
        second: Box::new(Pane::new_leaf()),
    };

    let promoted_sibling_path = tree.remove_leaf_at(&[false]).unwrap();
    assert_eq!(promoted_sibling_path, vec![false]);
    assert!(matches!(tree, Pane::Split { .. }));
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

    let next = tree.cycle(&[], AdjustDirection::Next).unwrap();
    assert_eq!(next, vec![]);

    let prev = tree.cycle(&[], AdjustDirection::Previous).unwrap();
    assert_eq!(prev, vec![]);
}

#[test]
fn test_cycle_nested_leaf() {
    let tree = Pane::new_split(layout::Direction::Horizontal);

    let next = tree.cycle(&[true], AdjustDirection::Next).unwrap();
    assert_eq!(next, vec![false]);

    let next = tree.cycle(&[false], AdjustDirection::Next).unwrap();
    assert_eq!(next, vec![true]);

    let prev = tree.cycle(&[true], AdjustDirection::Previous).unwrap();
    assert_eq!(prev, vec![false]);

    let prev = tree.cycle(&[false], AdjustDirection::Previous).unwrap();
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
        tree.cycle(&[true], AdjustDirection::Next).unwrap(),
        vec![false, true]
    );

    assert_eq!(
        tree.cycle(&[false, false, false], AdjustDirection::Next)
            .unwrap(),
        vec![true]
    );

    assert_eq!(
        tree.cycle(&[true], AdjustDirection::Previous).unwrap(),
        vec![false, false, false]
    );

    assert_eq!(
        tree.cycle(&[false, false, false], AdjustDirection::Previous)
            .unwrap(),
        vec![false, false, true]
    );
}

// Cycle fails when invalid path is provided
#[test]
fn test_cycle_invalid_leaf() {
    let tree = Pane::new_leaf();

    let result = tree.cycle(&[true], AdjustDirection::Next);
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

// Update image ID
#[test]
fn test_cycle_image_id() {
    let mut tree = Pane::new_split(layout::Direction::Horizontal);

    tree.cycle_image(&[true], 3, AdjustDirection::Next).unwrap();
    assert!(matches!(
        tree.get_node_at(&[true]).unwrap(),
        Pane::Leaf { image_id: 1 }
    ));

    tree.cycle_image(&[true], 3, AdjustDirection::Next).unwrap();
    assert!(matches!(
        tree.get_node_at(&[true]).unwrap(),
        Pane::Leaf { image_id: 2 }
    ));

    tree.cycle_image(&[true], 3, AdjustDirection::Next).unwrap();
    assert!(matches!(
        tree.get_node_at(&[true]).unwrap(),
        Pane::Leaf { image_id: 0 }
    ));

    tree.cycle_image(&[true], 3, AdjustDirection::Previous)
        .unwrap();
    assert!(matches!(
        tree.get_node_at(&[true]).unwrap(),
        Pane::Leaf { image_id: 2 }
    ));

    tree.cycle_image(&[true], 3, AdjustDirection::Previous)
        .unwrap();
    assert!(matches!(
        tree.get_node_at(&[true]).unwrap(),
        Pane::Leaf { image_id: 1 }
    ));

    tree.cycle_image(&[true], 3, AdjustDirection::Previous)
        .unwrap();
    assert!(matches!(
        tree.get_node_at(&[true]).unwrap(),
        Pane::Leaf { image_id: 0 }
    ));
}

// Navigate directions
#[test]
fn test_root_leaf_on_navigate_does_nothing() {
    let tree = Pane::new_leaf();

    let nextv = tree
        .navigate(&[], layout::Direction::Vertical, AdjustDirection::Next)
        .unwrap();
    assert_eq!(nextv, vec![]);

    let prevv = tree
        .navigate(&[], layout::Direction::Vertical, AdjustDirection::Previous)
        .unwrap();
    assert_eq!(prevv, vec![]);

    let nexth = tree
        .navigate(&[], layout::Direction::Horizontal, AdjustDirection::Next)
        .unwrap();
    assert_eq!(nexth, vec![]);

    let prevh = tree
        .navigate(
            &[],
            layout::Direction::Horizontal,
            AdjustDirection::Previous,
        )
        .unwrap();
    assert_eq!(prevh, vec![]);
}

#[test]
fn test_root_hsplit_on_vnav_does_nothing() {
    let tree = Pane::new_split(layout::Direction::Horizontal);

    let nextv_fromleft = tree
        .navigate(&[true], layout::Direction::Vertical, AdjustDirection::Next)
        .unwrap();
    assert_eq!(nextv_fromleft, vec![true]);

    let prevv_fromleft = tree
        .navigate(
            &[true],
            layout::Direction::Vertical,
            AdjustDirection::Previous,
        )
        .unwrap();
    assert_eq!(prevv_fromleft, vec![true]);

    let nextv_fromright = tree
        .navigate(&[false], layout::Direction::Vertical, AdjustDirection::Next)
        .unwrap();
    assert_eq!(nextv_fromright, vec![false]);

    let prevv_fromright = tree
        .navigate(
            &[false],
            layout::Direction::Vertical,
            AdjustDirection::Previous,
        )
        .unwrap();
    assert_eq!(prevv_fromright, vec![false]);
}

#[test]
fn test_root_vsplit_on_hnav_does_nothing() {
    let tree = Pane::new_split(layout::Direction::Vertical);

    let nexth_fromtop = tree
        .navigate(
            &[true],
            layout::Direction::Horizontal,
            AdjustDirection::Next,
        )
        .unwrap();
    assert_eq!(nexth_fromtop, vec![true]);

    let prevh_fromtop = tree
        .navigate(
            &[true],
            layout::Direction::Horizontal,
            AdjustDirection::Previous,
        )
        .unwrap();
    assert_eq!(prevh_fromtop, vec![true]);

    let nexth_frombottom = tree
        .navigate(
            &[false],
            layout::Direction::Horizontal,
            AdjustDirection::Next,
        )
        .unwrap();
    assert_eq!(nexth_frombottom, vec![false]);

    let prevh_frombottom = tree
        .navigate(
            &[false],
            layout::Direction::Horizontal,
            AdjustDirection::Previous,
        )
        .unwrap();
    assert_eq!(prevh_frombottom, vec![false]);
}

#[test]
fn test_root_vsplit_on_vnav() {
    let tree = Pane::new_split(layout::Direction::Vertical);

    let nextv_fromtop = tree
        .navigate(&[true], layout::Direction::Vertical, AdjustDirection::Next)
        .unwrap();
    assert_eq!(nextv_fromtop, vec![false]);

    let prevv_fromtop = tree
        .navigate(
            &[true],
            layout::Direction::Vertical,
            AdjustDirection::Previous,
        )
        .unwrap();
    assert_eq!(prevv_fromtop, vec![true]);

    let nextv_frombottom = tree
        .navigate(&[false], layout::Direction::Vertical, AdjustDirection::Next)
        .unwrap();
    assert_eq!(nextv_frombottom, vec![false]);

    let prevv_frombottom = tree
        .navigate(
            &[false],
            layout::Direction::Vertical,
            AdjustDirection::Previous,
        )
        .unwrap();
    assert_eq!(prevv_frombottom, vec![true]);
}
