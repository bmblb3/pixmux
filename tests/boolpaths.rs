use pixmux::boolpaths::BoolPathOperations;

#[rstest::rstest]
#[case(vec![
        vec![],
    ],
    vec![
        vec![],
    ],
)] // leaf at root
#[case(vec![
        vec![true],
        vec![false],
    ],
    vec![
        vec![],
        vec![],
    ],
)] // branch at root
#[case(vec![
        vec![true, true],
        vec![true, false],
        vec![false],
    ],
    vec![
        vec![true],
        vec![true],
        vec![false],
    ],
)] // first-heavy branch at root
#[case(vec![
        vec![true],
        vec![false, true],
        vec![false, false],
    ],
    vec![
        vec![true],
        vec![false],
        vec![false],
    ],
)] // first-heavy branch at root
fn test_get_nearest_promoted_leaf(
    #[case] all_paths: Vec<Vec<bool>>,
    #[case] expecteds: Vec<Vec<bool>>,
) {
    for (remove_path, expected) in all_paths.iter().zip(expecteds) {
        let actual = BoolPathOperations::get_nearest_promoted_leaf(&all_paths, remove_path);
        assert_eq!(actual, expected);
    }
}
