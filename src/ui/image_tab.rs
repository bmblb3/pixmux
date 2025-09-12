use std::slice::Iter;

use pixmux::btree::BTreeNode;
use pixmux::panes::{PaneData, PaneType, SplitData, SplitDirection};
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::style::Color;
use ratatui::widgets::Block;
use ratatui::{Frame, layout};
use ratatui_image::{StatefulImage, picker};

use crate::app::App;

pub struct ImageTabUI;

impl ImageTabUI {
    fn render_pane(
        pane: &PaneType,
        paths: &mut Iter<Vec<bool>>,
        frame: &mut Frame,
        area: Rect,
        app: &App,
        picker: &picker::Picker,
    ) {
        match pane {
            BTreeNode::Leaf(PaneData { imagefile }) => {
                let block = Block::bordered();

                if app.current_pane_path.is_some()
                    && paths
                        .next()
                        .is_some_and(|x| *x == *app.current_pane_path.as_ref().unwrap())
                {
                    frame.render_widget(block.clone().style(Color::LightYellow), area);
                } else {
                    frame.render_widget(block.clone(), area);
                }

                let imagedir = app.imagedir_paths.get(app.current_datarow_index).unwrap();
                let imagefile = imagedir.join(imagefile);

                if imagefile.is_file() {
                    let image_source = image::ImageReader::open(imagefile)
                        .unwrap()
                        .decode()
                        .unwrap();
                    let mut image = picker.new_resize_protocol(image_source);

                    frame.render_stateful_widget(
                        StatefulImage::default(),
                        block.inner(area),
                        &mut image,
                    );
                }
            }
            BTreeNode::Branch {
                first,
                second,
                data: SplitData { pct, direction },
                ..
            } => {
                let constraints = vec![
                    Constraint::Percentage(*pct as u16),
                    Constraint::Percentage((100 - pct) as u16),
                ];
                let chunks = Layout::default()
                    .spacing(-1)
                    .direction(match *direction {
                        SplitDirection::Vertical => layout::Direction::Horizontal,
                        SplitDirection::Horizontal => layout::Direction::Vertical,
                    })
                    .constraints(constraints)
                    .split(area);
                Self::render_pane(first, paths, frame, chunks[0], app, picker);
                Self::render_pane(second, paths, frame, chunks[1], app, picker);
            }
        }
    }

    pub fn render(frame: &mut Frame, area: Rect, app: &App) {
        let picker = picker::Picker::from_query_stdio().unwrap();
        Self::render_pane(
            app.pane_tree.inner(),
            &mut app.pane_tree.get_spec().leaf_paths.iter(),
            frame,
            area,
            app,
            &picker,
        );
    }
}
