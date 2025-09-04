pub enum CycleDirection {
    Forward,
    Backward,
}

pub fn cycle_index(index: &mut usize, len: usize, direction: CycleDirection) {
    *index = match direction {
        CycleDirection::Forward => (*index + 1) % len,
        CycleDirection::Backward => (*index + len - 1) % len,
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_forward_cycle() {
        let test_cases = [(0, 3, 1), (1, 3, 2), (2, 3, 0)];

        for test_case in test_cases {
            let (mut index, len, expected) = test_case;
            cycle_index(&mut index, len, CycleDirection::Forward);
            assert_eq!(index, expected);
        }
    }

    #[test]
    fn test_backward_cycle() {
        let test_cases = [(0, 3, 2), (1, 3, 0), (2, 3, 1)];

        for test_case in test_cases {
            let (mut index, len, expected) = test_case;
            cycle_index(&mut index, len, CycleDirection::Backward);
            assert_eq!(index, expected);
        }
    }

    #[test]
    fn test_single_element() {
        let dirs = [CycleDirection::Forward, CycleDirection::Backward];

        for dir in dirs {
            let mut index = 0;
            cycle_index(&mut index, 1, dir);
            assert_eq!(index, 0);
        }
    }
}
