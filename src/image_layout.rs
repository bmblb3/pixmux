use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    widgets::{Block, Paragraph},
};

use crate::app::App;

#[allow(dead_code)]
pub enum ChildPanePosition {
    LeftOrTop,
    RightOrBottom,
}

#[derive(Debug, Default)]
#[allow(dead_code)]
pub enum Pane {
    #[default]
    Leaf,
    Split {
        direction: Direction,
        first: Box<Pane>,
        second: Box<Pane>,
    },
}

#[derive(Default)]
pub struct ImageLayout;

impl ImageLayout {
    fn render_imgpane(frame: &mut Frame, area: Rect, pane: &Pane) {
        match pane {
            Pane::Leaf => {
                frame.render_widget(Paragraph::new("Image Pane").block(Block::bordered()), area);
            }
            Pane::Split {
                direction,
                first,
                second,
            } => {
                let constraints = vec![Constraint::Percentage(50), Constraint::Percentage(50)];
                let chunks = Layout::default()
                    .direction(*direction)
                    .constraints(constraints)
                    .split(area);
                Self::render_imgpane(frame, chunks[0], first);
                Self::render_imgpane(frame, chunks[1], second);
            }
        }
    }

    pub fn render(frame: &mut Frame, area: Rect, app: &App) {
        Self::render_imgpane(frame, area, &app.root_imgpane);
    }
}
