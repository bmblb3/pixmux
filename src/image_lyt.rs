use ratatui::widgets::{Block, Paragraph};

#[derive(Default)]
pub struct ImageLayout;

impl ImageLayout {
    pub fn create_widget() -> Paragraph<'static> {
        Paragraph::new("Image content here").block(Block::bordered())
    }
}
