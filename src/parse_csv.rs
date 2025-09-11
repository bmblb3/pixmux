use std::path;

use color_eyre::Result;
use color_eyre::eyre::{self, OptionExt};

type CsvData = (Vec<String>, Vec<Vec<String>>, Vec<path::PathBuf>);

pub fn parse_csv(filepath: &path::PathBuf) -> Result<CsvData> {
    let mut rdr = csv::Reader::from_path(filepath)?;

    let headers: Vec<String> = rdr.headers()?.iter().map(String::from).collect();

    let rows: Vec<Vec<String>> = rdr
        .records()
        .collect::<Result<Vec<_>, _>>()?
        .into_iter()
        .map(|record| record.into_iter().map(String::from).collect())
        .collect();

    let underscore_index = headers
        .iter()
        .position(|h| h == "_")
        .ok_or_eyre("Missing \"_\" column")?;

    let headers: Vec<String> = headers
        .into_iter()
        .enumerate()
        .filter_map(|(i, h)| (i != underscore_index).then_some(h))
        .collect();
    if headers.is_empty() {
        return Err(eyre::eyre!("Missing data columns"));
    }

    let csv_dir = filepath
        .parent()
        .ok_or_eyre("Could not determine parent directory of CSV file")?;

    let row_dirs: Vec<path::PathBuf> = rows
        .iter()
        .map(|row| csv_dir.join(&row[underscore_index]))
        .collect();

    for dir in &row_dirs {
        if !dir.is_dir() {
            return Err(eyre::eyre!(
                "The \"_\" column in the CSV must correspond to dirs!"
            ));
        }
    }

    let rows: Vec<Vec<String>> = rows
        .into_iter()
        .map(|row| {
            row.into_iter()
                .enumerate()
                .filter_map(|(i, val)| (i != underscore_index).then_some(val))
                .collect()
        })
        .collect();

    Ok((headers, rows, row_dirs))
}
