pub enum AdjustDirection {
    Next,
    Previous,
}

pub fn cycle_index(index: usize, len: usize, direction: AdjustDirection) -> usize {
    match direction {
        AdjustDirection::Next => (index + 1) % len,
        AdjustDirection::Previous => (index + len - 1) % len,
    }
}

pub fn step_index(index: usize, len: usize, direction: AdjustDirection) -> usize {
    match direction {
        AdjustDirection::Next => (index + 1).min(len - 1),
        AdjustDirection::Previous => index.saturating_sub(1),
    }
}
