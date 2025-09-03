use std::fs::File;
use std::io::Read as _;

use color_eyre::Result;

type CsvData = (Vec<String>, Vec<Vec<String>>, Vec<std::path::PathBuf>);

pub fn parse_csv(path: &str) -> Result<CsvData> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let mut lines = contents.lines();
    let header_line = lines
        .next()
        .ok_or_else(|| color_eyre::eyre::eyre!("Empty CSV file"))?;

    let all_headers: Vec<&str> = header_line.split(',').map(|s| s.trim()).collect();
    let underscore_col_idx = all_headers.iter().position(|&h| h == "_");

    let headers: Vec<String> = all_headers
        .iter()
        .enumerate()
        .filter(|(i, _)| Some(*i) != underscore_col_idx)
        .map(|(_, &h)| h.to_string())
        .collect();

    let csv_dir = std::path::Path::new(path)
        .parent()
        .unwrap_or(std::path::Path::new("."));
    let mut dir_paths = Vec::new();
    let mut table = Vec::new();

    for line in lines {
        let row: Vec<&str> = line.split(',').map(|s| s.trim()).collect();

        if let Some(idx) = underscore_col_idx
            && idx < row.len()
        {
            let rel_path = row[idx];
            let abs_path = csv_dir.join(rel_path);
            dir_paths.push(abs_path);
        }

        let filtered_row: Vec<String> = row
            .iter()
            .enumerate()
            .filter(|(i, _)| Some(*i) != underscore_col_idx)
            .map(|(_, &cell)| cell.to_string())
            .collect();
        table.push(filtered_row);
    }

    Ok((headers, table, dir_paths))
}
