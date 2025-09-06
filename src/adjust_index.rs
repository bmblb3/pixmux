pub enum AdjustDirection {
    Forward,
    Backward,
}

pub fn cycle_index(index: usize, len: usize, direction: AdjustDirection) -> usize {
    match direction {
        AdjustDirection::Forward => (index + 1) % len,
        AdjustDirection::Backward => (index + len - 1) % len,
    }
}

pub fn step_index(index: usize, len: usize, direction: AdjustDirection) -> usize {
    match direction {
        AdjustDirection::Forward => (index + 1).min(len - 1),
        AdjustDirection::Backward => index.saturating_sub(1),
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
            assert_eq!(cycle_index(index, len, AdjustDirection::Forward), expected);
        }
    }

    #[test]
    fn test_backward_cycle() {
        let test_cases = [(0, 3, 2), (1, 3, 0), (2, 3, 1)];

        for test_case in test_cases {
            let (index, len, expected) = test_case;
            assert_eq!(cycle_index(index, len, AdjustDirection::Backward), expected);
        }
    }

    #[test]
    fn test_single_element_cycle() {
        let dirs = [AdjustDirection::Forward, AdjustDirection::Backward];

        for dir in dirs {
            assert_eq!(cycle_index(0, 1, dir), 0);
        }
    }

    #[test]
    fn test_forward_step() {
        let test_cases = [(0, 3, 1), (1, 3, 2), (2, 3, 2)];

        for test_case in test_cases {
            let (index, len, expected) = test_case;
            assert_eq!(step_index(index, len, AdjustDirection::Forward), expected);
        }
    }

    #[test]
    fn test_backward_step() {
        let test_cases = [(0, 3, 0), (1, 3, 0), (2, 3, 1)];

        for test_case in test_cases {
            let (index, len, expected) = test_case;
            assert_eq!(step_index(index, len, AdjustDirection::Backward), expected);
        }
    }

    #[test]
    fn test_single_element_step() {
        let dirs = [AdjustDirection::Forward, AdjustDirection::Backward];

        for dir in dirs {
            assert_eq!(step_index(0, 1, dir), 0);
        }
    }
}
