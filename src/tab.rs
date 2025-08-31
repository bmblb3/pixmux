use ratatui::{
    Frame,
    layout::Rect,
    style::{Color, Style},
    widgets::{Block, Paragraph, Table, Tabs, Widget},
};

use crate::App;

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum Tab {
    #[default]
    Data,
    Image,
}

impl Tab {
    pub fn title(&self) -> &'static str {
        match self {
            Tab::Data => "Data",
            Tab::Image => "Image",
        }
    }

    pub fn next(&self) -> Self {
        match self {
            Tab::Data => Tab::Image,
            Tab::Image => Tab::Data,
        }
    }

    pub fn previous(&self) -> Self {
        self.next()
    }

    pub fn all() -> Vec<Self> {
        vec![Tab::Data, Tab::Image]
    }

    pub fn titles() -> Vec<&'static str> {
        Self::all().iter().map(|tab| tab.title()).collect()
    }

    pub fn to_index(self) -> usize {
        match self {
            Tab::Data => 0,
            Tab::Image => 1,
        }
    }

    pub fn render_navbar(&self, frame: &mut ratatui::Frame, area: ratatui::layout::Rect) {
        let tabs = Tabs::new(Self::titles())
            .block(Block::bordered())
            .select(self.to_index())
            .highlight_style(Style::default().fg(Color::Yellow));
        frame.render_widget(tabs, area);
    }

    pub fn create_widget(&self, app: &App) -> TabWidget {
        match self {
            Tab::Data => {
                use ratatui::widgets::{Cell, Row, Table};
                let rows = app
                    .table
                    .iter()
                    .map(|row| {
                        Row::new(
                            row.iter()
                                .map(|cell| Cell::from(cell.clone()))
                                .collect::<Vec<_>>(),
                        )
                    })
                    .collect::<Vec<_>>();
                TabWidget::DataTable(
                    Table::new(rows, app.headers.iter().map(|_| 20).collect::<Vec<_>>())
                        .header(Row::new(
                            app.headers
                                .iter()
                                .map(|h| Cell::from(h.clone()))
                                .collect::<Vec<_>>(),
                        ))
                        .block(Block::bordered()),
                )
            }
            Tab::Image => {
                TabWidget::Image(Paragraph::new("Image content here").block(Block::bordered()))
            }
        }
    }

    pub fn render(&self, frame: &mut Frame, area: Rect, app: &App) {
        frame.render_widget(self.create_widget(app), area);
    }
}

pub enum TabWidget {
    DataTable(Table<'static>),
    Image(Paragraph<'static>),
}

impl Widget for TabWidget {
    fn render(self, area: ratatui::layout::Rect, buf: &mut ratatui::buffer::Buffer) {
        match self {
            TabWidget::DataTable(widget) => widget.render(area, buf),
            TabWidget::Image(widget) => widget.render(area, buf),
        }
    }
}
