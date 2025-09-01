use ratatui::{
    Frame,
    layout::{Constraint, Layout, Rect},
    widgets::Block,
};

#[derive(Default)]
pub struct ImageLayout;

impl ImageLayout {
    pub fn render_layout(frame: &mut Frame, area: Rect) {
        let [left, right] = Layout::horizontal([Constraint::Fill(1); 2]).areas(area);
        let [top_right, bottom_right] = Layout::vertical([Constraint::Fill(1); 2]).areas(right);

        frame.render_widget(Block::bordered(), left);
        frame.render_widget(Block::bordered(), top_right);
        frame.render_widget(Block::bordered(), bottom_right);
    }
}
