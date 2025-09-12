use pixmux::btree::BTreeNode;
use pixmux::panes::{Pane, PaneData, SplitData, SplitDirection};

#[test]
fn test_init() {
    let pane = Pane::default();

    let coll_spec = pane.get_spec();

    assert!(matches!(pane.inner(), BTreeNode::Leaf(PaneData { .. })));
    assert_eq!(coll_spec.leaf_paths, vec![vec![]]);
    assert!(matches!(coll_spec.leaf_data.len(), 1));
    assert!(matches!(
        coll_spec.leaf_data.first().unwrap(),
        PaneData { .. }
    ));
    assert!(matches!(coll_spec.branch_data.len(), 0));
}

#[test]
fn test_branch() {
    let mut pane = Pane::default();
    pane.vsplit(&[]).unwrap();

    let coll_spec = pane.get_spec();

    assert_eq!(coll_spec.leaf_paths, vec![vec![true], vec![false]]);
    assert!(matches!(coll_spec.leaf_data.len(), 2));
    assert!(matches!(
        coll_spec.leaf_data.first().unwrap(),
        PaneData { .. }
    ));
    assert!(matches!(
        coll_spec.leaf_data.get(1).unwrap(),
        PaneData { .. }
    ));
    assert!(matches!(coll_spec.branch_data.len(), 1));
    assert!(matches!(
        coll_spec.branch_data.first().unwrap(),
        SplitData { .. }
    ));
}

#[test]
fn test_splits() {
    let mut pane = Pane::default();
    pane.vsplit(&[]).unwrap();
    pane.hsplit(&[false]).unwrap();

    let coll_spec = pane.get_spec();

    assert_eq!(
        coll_spec.leaf_paths,
        vec![vec![true], vec![false, true], vec![false, false]]
    );
    assert!(matches!(coll_spec.leaf_data.len(), 3));
    assert!(matches!(coll_spec.branch_data.len(), 2));
    assert!(matches!(
        coll_spec.branch_data.first().unwrap(),
        SplitData {
            direction: SplitDirection::Vertical,
            ..
        }
    ));
    assert!(matches!(
        coll_spec.branch_data.get(1).unwrap(),
        SplitData {
            direction: SplitDirection::Horizontal,
            ..
        }
    ));
}

#[test]
fn test_remove() {
    let mut pane = Pane::default();

    pane.remove(&[]).unwrap();
    let coll_spec = pane.get_spec();
    assert_eq!(coll_spec.leaf_paths, vec![vec![]]);
}

#[test]
fn test_navigate() {
    let mut pane = Pane::default();
    pane.vsplit(&[]).unwrap();

    assert_eq!(pane.next(&None).unwrap(), vec![true]);
    assert_eq!(pane.next(&Some(vec![true])).unwrap(), vec![false]);
    assert_eq!(pane.next(&Some(vec![false])).unwrap(), vec![true]);
    assert_eq!(pane.prev(&None).unwrap(), vec![false]);
    assert_eq!(pane.prev(&Some(vec![false])).unwrap(), vec![true]);
    assert_eq!(pane.prev(&Some(vec![true])).unwrap(), vec![false]);
}
