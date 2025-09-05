use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::style::{Color, Style};
use ratatui::widgets::{Block, Tabs};

use super::ImageLayout;
use crate::App;
use crate::data_table::DataTable;

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum Tab {
    #[default]
    Data,
    Image,
}

impl Tab {
    pub fn next(&self) -> Self {
        match self {
            Tab::Data => Tab::Image,
            Tab::Image => Tab::Data,
        }
    }

    pub fn titles() -> Vec<&'static str> {
        let order = [Tab::Data, Tab::Image];
        order
            .iter()
            .map(|tab| match tab {
                Tab::Data => "Data",
                Tab::Image => "Image",
            })
            .collect()
    }

    pub fn to_index(self) -> usize {
        match self {
            Tab::Data => 0,
            Tab::Image => 1,
        }
    }

    pub fn render_navbar(&self, frame: &mut ratatui::Frame, area: ratatui::layout::Rect) {
        let tabs = Tabs::new(Self::titles())
            .block(Block::bordered())
            .select(self.to_index())
            .highlight_style(Style::default().fg(Color::Yellow));
        frame.render_widget(tabs, area);
    }

    pub fn render(&self, frame: &mut Frame, area: Rect, app: &App) {
        match self {
            Tab::Data => {
                frame.render_widget(DataTable::create_widget(app), area);
            }
            Tab::Image => {
                ImageLayout::render(frame, area, app);
            }
        }
    }
}
