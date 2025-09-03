use std::path;

use color_eyre::Result;
use color_eyre::eyre::{self, OptionExt};

type CsvData = (Vec<String>, Vec<Vec<String>>, Vec<path::PathBuf>);

pub fn parse_csv(filepath: &path::PathBuf) -> Result<CsvData> {
    let csv_dir = filepath
        .parent()
        .ok_or_eyre("Could not determine parent directory of CSV file")?;

    let mut rdr = csv::Reader::from_path(filepath)?;

    let headers: Vec<String> = rdr.headers()?.iter().map(|f| f.to_string()).collect();

    headers
        .iter()
        .any(|h| h == "_")
        .then_some(())
        .ok_or_eyre("Headers must contain underscore")?;

    headers
        .iter()
        .any(|h| h != "_")
        .then_some(())
        .ok_or_eyre("Headers must contain atleast one non-underscore column (for data)")?;

    let records: Vec<csv::StringRecord> = rdr.records().collect::<Result<_, _>>()?;
    let rows: Vec<Vec<String>> = records
        .iter()
        .map(|str_record| Ok(str_record.iter().map(|val| val.to_string()).collect()))
        .collect::<Result<Vec<Vec<String>>, csv::Error>>()?;

    let underscore_index = headers.iter().position(|h| h == "_").unwrap();
    let headers: Vec<String> = headers
        .iter()
        .enumerate()
        .filter(|(i, _)| *i != underscore_index)
        .map(|(_, h)| h.to_string())
        .collect();

    let row_dirs: Vec<path::PathBuf> = rows
        .iter()
        .map(|vec_str| {
            Ok(csv_dir.join(
                vec_str
                    .get(underscore_index)
                    .ok_or_else(|| eyre::eyre!(""))?,
            ))
        })
        .collect::<Result<_, eyre::Report>>()?;
    if !row_dirs.iter().all(|d| d.is_dir()) {
        return Err(eyre::eyre!(
            "The \"_\" column in the CSV must correspond to dirs!"
        ));
    }

    let rows: Vec<Vec<String>> = rows
        .iter()
        .map(|vec_str| {
            Ok(vec_str
                .iter()
                .enumerate()
                .filter(|(i, _)| *i != underscore_index)
                .map(|(_, val)| val.to_string())
                .collect())
        })
        .collect::<Result<Vec<Vec<String>>, csv::Error>>()?;

    Ok((headers, rows, row_dirs))
}

#[cfg(test)]
mod tests {
    use std::io::Write;

    use super::*;

    fn parse_csv_string(csv_content: &str) -> Result<CsvData> {
        let mut file = tempfile::NamedTempFile::new().unwrap();
        file.write_all(csv_content.as_bytes()).unwrap();
        parse_csv(&file.path().to_path_buf())
    }

    #[test]
    fn test_malformed_csv() {
        let result = parse_csv_string(
            "\
oneheader,
firstvalue,extravalue",
        );
        assert!(result.is_err());
    }

    #[test]
    fn test_empty_csv() {
        let result = parse_csv_string("");
        assert!(result.is_err());
    }

    #[test]
    fn test_headers_only_no_data() {
        let result = parse_csv_string("only,headers");
        assert!(result.is_err());
    }

    #[test]
    fn test_no_underscore_in_headers() {
        let result = parse_csv_string(
            "\
    header,not_underscore
    value1,value2",
        );
        assert!(result.is_err());
    }

    #[test]
    fn test_only_underscore_columns() {
        let result = parse_csv_string(
            "\
    _
    value",
        );
        assert!(result.is_err());
    }

    #[test]
    fn test_not_a_dir() {
        let result = parse_csv_string(
            "\
header,_
value,not_a_dir",
        );
        assert!(result.is_err());
    }

    #[test]
    fn test_ok() {
        let tmpdir = tempfile::TempDir::new().unwrap();
        let created_dir_fullpath = tmpdir.path().to_str().unwrap().to_string();
        let created_dir = tmpdir.path().file_name().unwrap().to_str().unwrap();
        let csv_content = format!(
            "\
header,_
value,{}",
            created_dir
        );
        let (headers, rows, row_dirs) = parse_csv_string(&csv_content).unwrap();
        assert_eq!(headers, vec!["header"]);
        assert_eq!(row_dirs, vec![created_dir_fullpath]);
        assert_eq!(rows, vec![vec!["value"]]);
    }
}
