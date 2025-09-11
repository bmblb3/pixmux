use pixmux::{AdjustDirection, cycle_index, step_index};

#[test]
fn test_forward_cycle() {
    let test_cases = [(0, 3, 1), (1, 3, 2), (2, 3, 0)];

    for test_case in test_cases {
        let (index, len, expected) = test_case;
        assert_eq!(cycle_index(index, len, AdjustDirection::Next), expected);
    }
}

#[test]
fn test_backward_cycle() {
    let test_cases = [(0, 3, 2), (1, 3, 0), (2, 3, 1)];

    for test_case in test_cases {
        let (index, len, expected) = test_case;
        assert_eq!(cycle_index(index, len, AdjustDirection::Previous), expected);
    }
}

#[test]
fn test_single_element_cycle() {
    let dirs = [AdjustDirection::Next, AdjustDirection::Previous];

    for dir in dirs {
        assert_eq!(cycle_index(0, 1, dir), 0);
    }
}

#[test]
fn test_forward_step() {
    let test_cases = [(0, 3, 1), (1, 3, 2), (2, 3, 2)];

    for test_case in test_cases {
        let (index, len, expected) = test_case;
        assert_eq!(step_index(index, len, AdjustDirection::Next), expected);
    }
}

#[test]
fn test_backward_step() {
    let test_cases = [(0, 3, 0), (1, 3, 0), (2, 3, 1)];

    for test_case in test_cases {
        let (index, len, expected) = test_case;
        assert_eq!(step_index(index, len, AdjustDirection::Previous), expected);
    }
}

#[test]
fn test_single_element_step() {
    let dirs = [AdjustDirection::Next, AdjustDirection::Previous];

    for dir in dirs {
        assert_eq!(step_index(0, 1, dir), 0);
    }
}
