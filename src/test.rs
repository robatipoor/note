use crate::Notes;
use std::panic;
use std::fs::{self, File};

#[test]
fn get_line_test() {
    run_test("get-line", |p| {
        let note = Notes::new(p).append_line("hi").get_line(0);
        assert_eq!(note, "hi");
    });
}
#[test]
fn get_range_lines_test() {
    run_test("get-range", |p| {
        let note = Notes::new(p)
            .append_line("hi")
            .append_line("bye")
            .get_range_lines(0..=1);
        assert_eq!(note, "hi\nbye\n");
    });
}

#[test]
fn delete_range_lines_test() {
    run_test("delete-range", |p| {
        let note = Notes::new(p)
            .append_line("hi")
            .append_line("bye")
            .append_line("how")
            .append_line("get")
            .delete_range_lines(0..=1);
        assert_eq!(note.count_lines(), 2);
        assert_eq!(note.get_line(0), "how");
    })
}

#[test]
fn delete_line_test() {
    run_test("delete-line", |p| {
        let note = Notes::new(p)
            .append_line("hi")
            .append_line("bye")
            .delete_line(0);
        assert_eq!(note.get_line(0), "byes");
    })
}

fn run_test<T>(file: &str, test: T)
where
    T: FnOnce(&str) -> () + panic::UnwindSafe,
{
    File::create(file).unwrap();
    let _result = panic::catch_unwind(||{
        test(file);
    });
    fs::remove_file(file).unwrap();
}
