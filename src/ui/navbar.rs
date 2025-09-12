use ratatui::style::{Color, Style, Stylize};
use ratatui::widgets::Tabs;

use crate::App;

pub struct NavBarUI;

impl NavBarUI {
    pub fn render(frame: &mut ratatui::Frame, area: ratatui::layout::Rect, app: &App) {
        let tabs = Tabs::new(pixmux::Tab::titles())
            .select(app.current_tab.to_index())
            .style(Style::default().bg(Color::White).fg(Color::Black).bold())
            .highlight_style(Style::default().fg(Color::Yellow).bold())
            .padding(" | ", " | ")
            .divider(" ");
        frame.render_widget(tabs, area);
    }
}
