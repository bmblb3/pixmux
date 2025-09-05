use std::ptr;

use pixmux::Pane;
use ratatui::Frame;
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::style::Color;
use ratatui::widgets::Block;
use ratatui_image::{StatefulImage, picker};

use crate::app::App;

pub struct ImageTabUI;

impl ImageTabUI {
    fn render_pane(pane: &Pane, frame: &mut Frame, area: Rect, app: &App, picker: &picker::Picker) {
        match pane {
            Pane::Leaf { image_id } => {
                let block = Block::bordered();

                let current_pane = app.pane_tree.get_node_at(&app.current_pane_path).unwrap();
                if ptr::eq(current_pane, pane) {
                    frame.render_widget(block.clone().style(Color::LightYellow), area);
                } else {
                    frame.render_widget(block.clone(), area);
                }

                let imagefile = app.get_fullimgpath(image_id, &app.current_datarow_index);
                if let Some(f) = imagefile {
                    let image_source = image::ImageReader::open(f).unwrap().decode().unwrap();
                    let mut image = picker.new_resize_protocol(image_source);

                    frame.render_stateful_widget(
                        StatefulImage::default(),
                        block.inner(area),
                        &mut image,
                    );
                }
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
                Self::render_pane(first, frame, chunks[0], app, picker);
                Self::render_pane(second, frame, chunks[1], app, picker);
            }
        }
    }

    pub fn render(frame: &mut Frame, area: Rect, app: &App) {
        let picker = picker::Picker::from_query_stdio().unwrap();
        Self::render_pane(&app.pane_tree, frame, area, app, &picker);
    }
}
