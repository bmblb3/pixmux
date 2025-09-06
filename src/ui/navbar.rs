use ratatui::style::{Color, Style};
use ratatui::widgets::{Block, Tabs};

use crate::App;

pub struct NavBarUI;

impl NavBarUI {
    pub fn render(frame: &mut ratatui::Frame, area: ratatui::layout::Rect, app: &App) {
        let tabs = Tabs::new(pixmux::Tab::titles())
            .block(Block::bordered())
            .select(app.current_tab.to_index())
            .highlight_style(Style::default().fg(Color::Yellow));
        frame.render_widget(tabs, area);
    }
}
