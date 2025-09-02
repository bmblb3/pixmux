use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::Color,
    widgets::{Block, BorderType},
};
use ratatui_image::{StatefulImage, picker::Picker};

use crate::app::App;

#[allow(dead_code)]
pub enum ChildPanePosition {
    LeftOrTop,
    RightOrBottom,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub enum Pane {
    Leaf {
        image_id: usize,
    },
    Split {
        direction: Direction,
        pct: u8,
        first: Box<Pane>,
        second: Box<Pane>,
    },
}

impl Default for Pane {
    fn default() -> Self {
        Self::Leaf { image_id: 0 }
    }
}

impl Pane {
    pub fn leaf() -> Self {
        Self::Leaf { image_id: 0 }
    }

    pub fn split(direction: Direction) -> Self {
        Self::Split {
            direction,
            pct: 50,
            first: Box::new(Pane::leaf()),
            second: Box::new(Pane::leaf()),
        }
    }
}

#[derive(Default)]
pub struct ImageLayout;

impl ImageLayout {
    fn render_imgpane(
        frame: &mut Frame,
        area: Rect,
        pane: &Pane,
        pane_enum: &mut usize,
        app: &App,
    ) {
        match pane {
            Pane::Leaf { image_id } => {
                let block = Block::bordered().border_type(BorderType::QuadrantInside);
                let imagefile = app.get_fullimgpath(image_id, &app.current_row_index);
                match imagefile {
                    Some(f) => {
                        let picker = Picker::from_query_stdio().unwrap();
                        let image_source = image::ImageReader::open(f).unwrap().decode().unwrap();
                        let mut image = picker.new_resize_protocol(image_source);
                        frame.render_widget(block.clone().style(Color::Yellow), area);
                        frame.render_stateful_widget(
                            StatefulImage::default(),
                            block.inner(area),
                            &mut image,
                        );
                    }
                    _ => {
                        frame.render_widget(block, area);
                    }
                }

                *pane_enum += 1;
            }
            Pane::Split {
                direction,
                pct,
                first,
                second,
                ..
            } => {
                let constraints = vec![
                    Constraint::Percentage(*pct as u16),
                    Constraint::Percentage((100 - pct) as u16),
                ];
                let chunks = Layout::default()
                    .spacing(-1)
                    .direction(*direction)
                    .constraints(constraints)
                    .split(area);
                Self::render_imgpane(frame, chunks[0], first, pane_enum, app);
                Self::render_imgpane(frame, chunks[1], second, pane_enum, app);
            }
        }
    }

    pub fn render(frame: &mut Frame, area: Rect, app: &App) {
        let mut imgpane_enum = 0;
        Self::render_imgpane(frame, area, &app.root_imgpane, &mut imgpane_enum, app);
    }
}
