pub enum StepDirection {
    Forward,
    Backward,
}

pub fn step_index(index: usize, len: usize, direction: StepDirection) -> usize {
    match direction {
        StepDirection::Forward => (index + 1).min(len - 1),
        StepDirection::Backward => index.saturating_sub(1),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_forward_step() {
        let test_cases = [(0, 3, 1), (1, 3, 2), (2, 3, 2)];

        for test_case in test_cases {
            let (index, len, expected) = test_case;
            assert_eq!(step_index(index, len, StepDirection::Forward), expected);
        }
    }

    #[test]
    fn test_backward_step() {
        let test_cases = [(0, 3, 0), (1, 3, 0), (2, 3, 1)];

        for test_case in test_cases {
            let (index, len, expected) = test_case;
            assert_eq!(step_index(index, len, StepDirection::Backward), expected);
        }
    }

    #[test]
    fn test_single_element() {
        let dirs = [StepDirection::Forward, StepDirection::Backward];

        for dir in dirs {
            assert_eq!(step_index(0, 1, dir), 0);
        }
    }
}
