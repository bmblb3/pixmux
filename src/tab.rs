#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Tab {
    Data,
    Image,
}

impl Tab {
    pub fn title(&self) -> &'static str {
        match self {
            Tab::Data => "Data",
            Tab::Image => "Image",
        }
    }

    pub fn next(&self) -> Self {
        match self {
            Tab::Data => Tab::Image,
            Tab::Image => Tab::Data,
        }
    }

    pub fn previous(&self) -> Self {
        self.next()
    }

    pub fn all() -> Vec<Self> {
        vec![Tab::Data, Tab::Image]
    }

    pub fn titles() -> Vec<&'static str> {
        Self::all().iter().map(|tab| tab.title()).collect()
    }

    pub fn to_index(self) -> usize {
        match self {
            Tab::Data => 0,
            Tab::Image => 1,
        }
    }

    pub fn content(&self) -> &'static str {
        match self {
            Tab::Data => "Data content here",
            Tab::Image => "Image content here",
        }
    }
}
