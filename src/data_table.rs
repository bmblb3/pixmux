use ratatui::{layout::Constraint, style::Stylize, widgets::Table};

use crate::App;

#[derive(Default)]
pub struct DataTable;

impl DataTable {
    pub fn new() -> Self {
        Self
    }

    pub fn create_widget(&self, app: &App) -> Table<'static> {
        let collen = app.headers.len();
        let constraints = vec![Constraint::Length(20); collen];

        use ratatui::widgets::{Block, Borders, Cell, Row, Table};
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

        Table::new(rows, constraints)
            .header(
                Row::new(
                    app.headers
                        .iter()
                        .map(|h| Cell::from(h.clone()))
                        .collect::<Vec<_>>(),
                )
                .bold()
                .gray()
                .underlined(),
            )
            .block(Block::default().borders(Borders::ALL))
    }
}
