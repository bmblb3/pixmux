use ratatui::{
    layout::Constraint,
    style::Stylize,
    widgets::{Block, Borders, Cell, Row, Table},
};

use crate::App;

#[derive(Default)]
pub struct DataTable;

impl DataTable {
    pub fn create_widget(app: &App) -> Table<'static> {
        let collen = app.headers.len();
        let constraints = vec![Constraint::Length(20); collen];

        let rows = app
            .table
            .iter()
            .enumerate()
            .map(|(index, row)| {
                let row_cells = row
                    .iter()
                    .map(|cell| Cell::from(cell.clone()))
                    .collect::<Vec<_>>();
                let mut table_row = Row::new(row_cells);
                if index == app.current_row_index as usize {
                    table_row = table_row.reversed();
                }
                table_row
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
