use std::io::Write;

use pixmux::parse_csv;

#[test]
fn test_ok_onerow_onedatacolumn() {
    let tmpdir = tempfile::TempDir::new().unwrap();
    let created_dir_fullpath = tmpdir.path().to_str().unwrap().to_string();
    let created_dir = tmpdir.path().file_name().unwrap().to_str().unwrap();
    let csv_content = format!(
        "\
onedatacol,_
onedatavalue,{}",
        created_dir
    );
    let mut file = tempfile::NamedTempFile::new().unwrap();
    file.write_all(csv_content.as_bytes()).unwrap();

    let result = parse_csv(&file.path().to_path_buf());
    let (headers, rows, row_dirs) = result.unwrap();

    assert_eq!(headers, vec!["onedatacol"]);
    assert_eq!(row_dirs, vec![created_dir_fullpath]);
    assert_eq!(rows, vec![vec!["onedatavalue"]]);
}

#[test]
fn test_ok_bigger_csv() {
    let tmpdir1 = tempfile::TempDir::new().unwrap();
    let created_dir_fullpath1 = tmpdir1.path().to_str().unwrap().to_string();
    let created_dir1 = tmpdir1.path().file_name().unwrap().to_str().unwrap();
    let tmpdir2 = tempfile::TempDir::new().unwrap();
    let created_dir_fullpath2 = tmpdir2.path().to_str().unwrap().to_string();
    let created_dir2 = tmpdir2.path().file_name().unwrap().to_str().unwrap();
    let csv_content = format!(
        "\
firstheader,secondheader,_
value11,value12,{created_dir1}
value21,value22,{created_dir2}",
    );
    let mut file = tempfile::NamedTempFile::new().unwrap();
    file.write_all(csv_content.as_bytes()).unwrap();

    let result = parse_csv(&file.path().to_path_buf());
    let (headers, rows, row_dirs) = result.unwrap();

    assert_eq!(headers, vec!["firstheader", "secondheader"]);
    assert_eq!(row_dirs, vec![created_dir_fullpath1, created_dir_fullpath2]);
    assert_eq!(
        rows,
        vec![vec!["value11", "value12"], vec!["value21", "value22"]]
    );
}

#[test]
fn test_err_empty_csv() {
    let csv_content = "";
    let mut file = tempfile::NamedTempFile::new().unwrap();
    file.write_all(csv_content.as_bytes()).unwrap();

    let result = parse_csv(&file.path().to_path_buf());

    assert!(result.is_err());
}

#[test]
fn test_err_only_headers_no_rows() {
    let csv_content = "only,headers";
    let mut file = tempfile::NamedTempFile::new().unwrap();
    file.write_all(csv_content.as_bytes()).unwrap();

    let result = parse_csv(&file.path().to_path_buf());

    assert!(result.is_err());
}

#[test]
fn test_err_no_underscore_column() {
    let csv_content = "\
no,underscore,column
valu1,value2,value3";
    let mut file = tempfile::NamedTempFile::new().unwrap();
    file.write_all(csv_content.as_bytes()).unwrap();

    let result = parse_csv(&file.path().to_path_buf());

    assert!(result.is_err());
}

#[test]
fn test_err_no_data_column() {
    let tmpdir = tempfile::TempDir::new().unwrap();
    let created_dir = tmpdir.path().file_name().unwrap().to_str().unwrap();
    let csv_content = format!(
        "\
_
{}",
        created_dir
    );
    let mut file = tempfile::NamedTempFile::new().unwrap();
    file.write_all(csv_content.as_bytes()).unwrap();

    let result = parse_csv(&file.path().to_path_buf());

    assert!(result.is_err());
}

#[test]
fn test_err_notadir_in_underscore_colum() {
    let csv_content = "\
datacol,_
value,not_a_dir";
    let mut file = tempfile::NamedTempFile::new().unwrap();
    file.write_all(csv_content.as_bytes()).unwrap();

    let result = parse_csv(&file.path().to_path_buf());

    assert!(result.is_err());
}
