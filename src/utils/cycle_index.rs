pub enum CycleDirection {
    Forward,
    Backward,
}

pub fn cycle_index(index: usize, len: usize, direction: CycleDirection) -> usize {
    match direction {
        CycleDirection::Forward => (index + 1) % len,
        CycleDirection::Backward => (index + len - 1) % len,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_forward_cycle() {
        let test_cases = [(0, 3, 1), (1, 3, 2), (2, 3, 0)];

        for test_case in test_cases {
            let (index, len, expected) = test_case;
            assert_eq!(cycle_index(index, len, CycleDirection::Forward), expected);
        }
    }

    #[test]
    fn test_backward_cycle() {
        let test_cases = [(0, 3, 2), (1, 3, 0), (2, 3, 1)];

        for test_case in test_cases {
            let (index, len, expected) = test_case;
            assert_eq!(cycle_index(index, len, CycleDirection::Backward), expected);
        }
    }

    #[test]
    fn test_single_element() {
        let dirs = [CycleDirection::Forward, CycleDirection::Backward];

        for dir in dirs {
            assert_eq!(cycle_index(0, 1, CycleDirection::Backward), 0);
        }
    }
}
