use ratatui::widgets::{Block, Paragraph, Widget};

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

    pub fn create_widget(&self) -> TabWidget {
        match self {
            Tab::Data => {
                TabWidget::Data(Paragraph::new("Data content here").block(Block::bordered()))
            }
            Tab::Image => {
                TabWidget::Image(Paragraph::new("Image content here").block(Block::bordered()))
            }
        }
    }

    pub fn render(&self, frame: &mut ratatui::Frame, area: ratatui::layout::Rect) {
        frame.render_widget(self.create_widget(), area);
    }
}

pub enum TabWidget {
    Data(Paragraph<'static>),
    Image(Paragraph<'static>),
}

impl Widget for TabWidget {
    fn render(self, area: ratatui::layout::Rect, buf: &mut ratatui::buffer::Buffer) {
        match self {
            TabWidget::Data(widget) => widget.render(area, buf),
            TabWidget::Image(widget) => widget.render(area, buf),
        }
    }
}
