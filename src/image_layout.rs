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

#[derive(Debug, Default, Clone)]
#[allow(dead_code)]
pub enum Pane {
    #[default]
    Leaf,
    Split {
        direction: Direction,
        pct: u8,
        image_id: usize,
        first: Box<Pane>,
        second: Box<Pane>,
    },
}

impl Pane {
    pub fn split(direction: Direction) -> Self {
        Self::Split {
            direction,
            image_id: 999,
            pct: 50,
            first: Box::new(Pane::Leaf),
            second: Box::new(Pane::Leaf),
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
        current_pane_id: &usize,
    ) {
        match pane {
            Pane::Leaf => {
                let imagefile = "/home/akucwh/techsim_root/nsa/fluidcfd/01_Ongoing/01_ATS/Increment/PI2526/VES-12345__DORA_Iteration_Work/930__cup_wing_doe/concept_1.2/doe.46/MB3/T_evap.png";
                let picker = Picker::from_query_stdio().unwrap();
                let image_source = image::ImageReader::open(imagefile)
                    .unwrap()
                    .decode()
                    .unwrap();
                let mut image = picker.new_resize_protocol(image_source);

                let block = Block::bordered().border_type(BorderType::QuadrantInside);
                if pane_enum == current_pane_id {
                    frame.render_widget(block.clone().style(Color::Yellow), area);
                    frame.render_stateful_widget(
                        StatefulImage::default(),
                        block.inner(area),
                        &mut image,
                    );
                } else {
                    frame.render_widget(block, area);
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
