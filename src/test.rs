use super::*;

#[test]
fn del_line_test() {
    let mut file: File = tempfile::tempfile().unwrap();
    writeln!(file, "test line 1").unwrap();
    writeln!(file, "test line 2").unwrap();
    writeln!(file, "test line 3").unwrap();
    file.seek(SeekFrom::Start(0)).unwrap();
    del_line(&mut file, 1..2);
    file.seek(SeekFrom::Start(0)).unwrap();
    assert_eq!(
        read_line(&mut file, 1).unwrap().trim(),
        "test line 2".to_owned()
    );
}

#[test]
fn read_line_test() {
    let mut file: File = tempfile::tempfile().unwrap();
    writeln!(file, "test line 1").unwrap();
    writeln!(file, "test line 2").unwrap();
    writeln!(file, "test line 3").unwrap();
    file.seek(SeekFrom::Start(0)).unwrap();
    assert_eq!(
        read_line(&mut file, 1).unwrap().trim(),
        "test line 1".to_owned()
    );
}

#[test]
fn parse_str_test() {
    assert_eq!(parse_str(2,"1..4").unwrap(), 1..5);
    assert_eq!(parse_str(2,"10..2"), None);
    assert_eq!(parse_str(2,"1 .."), None);
    assert_eq!(parse_str(2,"1 ..4"), None);
    assert_eq!(parse_str(2,"2..7"), Some(2..8));
    assert_eq!(parse_str(2,"10..2"), None);
    assert_eq!(parse_str(10,"3..").unwrap(), 3..11);
    assert_eq!(parse_str(1,"4").unwrap(), 4..5);
}
#[test]
fn count_line_test() {
    let mut file: File = tempfile::tempfile().unwrap();
    writeln!(file, "test line 1").unwrap();
    writeln!(file, "test line 2").unwrap();
    writeln!(file, "test line 3").unwrap();
    file.seek(SeekFrom::Start(0)).unwrap();
    assert_eq!(count_line(file), 3);
}
