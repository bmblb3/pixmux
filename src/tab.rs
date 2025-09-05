#[derive(Default)]
pub enum Tab {
    #[default]
    Data,
    Image,
}

impl Tab {
    pub fn cycle(&self) -> Self {
        match self {
            Tab::Data => Tab::Image,
            Tab::Image => Tab::Data,
        }
    }

    pub fn titles() -> Vec<&'static str> {
        vec!["Data", "Image"]
    }

    pub fn to_index(&self) -> usize {
        match self {
            Tab::Data => 0,
            Tab::Image => 1,
        }
    }
}
