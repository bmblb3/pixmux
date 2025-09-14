use ratatui::style::{Color, Style, Stylize};
use ratatui::widgets::Tabs;

use crate::App;

pub struct NavBarUI;

impl NavBarUI {
    pub fn render(frame: &mut ratatui::Frame, area: ratatui::layout::Rect, app: &App) {
        let tabs = Tabs::new(
            pixmux::Tab::titles()
                .iter()
                .map(|x| " ".to_string() + x + " "),
        )
        .select(app.current_tab.to_index())
        .style(Style::default().fg(Color::Yellow))
        .highlight_style(Style::default().bg(Color::Yellow).fg(Color::White).bold());
        frame.render_widget(tabs, area);
    }
}
