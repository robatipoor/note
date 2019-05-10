use crate::Notes;
use std::fs::File;

#[test]
fn get_line_test(){
    File::create("test.txt").unwrap();
    let note = Notes::new("test.txt").append_line("hi").get_line(0);
    assert_eq!(note,"hi");
}
#[test]
fn get_range_lines_test(){
    File::create("test.txt").unwrap();
    let note = Notes::new("test.txt").append_line("hi").append_line("bye").get_range_lines(0..2);
    assert_eq!(note,"hi\nbye\n");
}