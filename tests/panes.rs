use pixmux::panes::{Pane, PaneData};

#[test]
fn test_init() {
    let pane = Pane::default();

    let coll_spec = pane.get_spec();

    assert_eq!(coll_spec.leaf_paths, vec![vec![]]);
    assert!(matches!(
        coll_spec.leaf_data.first().unwrap(),
        PaneData { imagefile: _ }
    ));
}
