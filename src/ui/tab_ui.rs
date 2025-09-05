use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::style::{Color, Style};
use ratatui::widgets::{Block, Tabs};

use super::{ImageLayout, TableUI};
use crate::App;

pub struct TabUI;

impl TabUI {
    pub fn render_navbar(frame: &mut ratatui::Frame, area: ratatui::layout::Rect, app: &App) {
        let tabs = Tabs::new(pixmux::Tab::titles())
            .block(Block::bordered())
            .select(app.current_tab.to_index())
            .highlight_style(Style::default().fg(Color::Yellow));
        frame.render_widget(tabs, area);
    }

    pub fn render(frame: &mut Frame, area: Rect, app: &App) {
        match app.current_tab {
            pixmux::Tab::Data => {
                frame.render_widget(TableUI::create_widget(app), area);
            }
            pixmux::Tab::Image => {
                ImageLayout::render(frame, area, app);
            }
        }
    }
}
