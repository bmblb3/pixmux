use ratatui::widgets::Table;

use crate::App;

#[derive(Default)]
pub struct DataTable;

impl DataTable {
    pub fn new() -> Self {
        Self
    }

    pub fn create_widget(&self, app: &App) -> Table<'static> {
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

        Table::new(rows, app.headers.iter().map(|_| 20).collect::<Vec<_>>()).header(Row::new(
            app.headers
                .iter()
                .map(|h| Cell::from(h.clone()))
                .collect::<Vec<_>>(),
        ))
    }
}
