use ratatui::Frame;
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::style::Color;
use ratatui::widgets::{Block, BorderType};
use ratatui_image::StatefulImage;
use ratatui_image::picker::Picker;

use crate::app::App;
use crate::app::imgpane::Pane;

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
                        if *pane_enum == app.current_imgpane_id {
                            frame.render_widget(block.clone().style(Color::LightYellow), area);
                        } else {
                            frame.render_widget(block.clone(), area);
                        }
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
