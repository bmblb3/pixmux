use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::Color,
    widgets::Block,
};

use crate::app::App;

#[allow(dead_code)]
pub enum ChildPanePosition {
    LeftOrTop,
    RightOrBottom,
}

#[derive(Debug, Default, Clone)]
#[allow(dead_code)]
pub enum Pane {
    #[default]
    Leaf,
    Split {
        direction: Direction,
        pct: u8,
        first: Box<Pane>,
        second: Box<Pane>,
    },
}

#[derive(Default)]
pub struct ImageLayout;

impl ImageLayout {
    fn render_imgpane(
        frame: &mut Frame,
        area: Rect,
        pane: &Pane,
        pane_enum: &mut usize,
        current_pane_id: &usize,
    ) {
        match pane {
            Pane::Leaf => {
                if pane_enum == current_pane_id {
                    frame.render_widget(Block::bordered().style(Color::Yellow), area);
                } else {
                    frame.render_widget(Block::bordered(), area);
                }

                *pane_enum += 1;
            }
            Pane::Split {
                direction,
                pct,
                first,
                second,
            } => {
                let constraints = vec![
                    Constraint::Percentage(*pct as u16),
                    Constraint::Percentage((100 - pct) as u16),
                ];
                let chunks = Layout::default()
                    .direction(*direction)
                    .constraints(constraints)
                    .split(area);
                Self::render_imgpane(frame, chunks[0], first, pane_enum, current_pane_id);
                Self::render_imgpane(frame, chunks[1], second, pane_enum, current_pane_id);
            }
        }
    }

    pub fn render(frame: &mut Frame, area: Rect, app: &App) {
        let mut imgpane_enum = 0;
        Self::render_imgpane(
            frame,
            area,
            &app.root_imgpane,
            &mut imgpane_enum,
            &app.current_imgpane_id,
        );
    }
}
