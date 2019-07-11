use crate::FileHandler;
use colored::*;
use std::iter::FromIterator;
use std::ops::Range;
use std::path::Path;

pub struct Notes {
    file: FileHandler,
    lines: Vec<String>,
}

impl Notes {
    pub fn new<P: AsRef<Path>>(path: P) -> Self {
        let file = FileHandler::new(path);
        Notes {
            lines: file.read(),
            file,
        }
    }
    pub fn delete_line(mut self, index: usize) -> Self {
        self.lines.remove(index);
        self
    }
    pub fn delete_range_lines(mut self, range: Range<usize>) -> Self {
        self.lines.drain(range);
        self
    }
    pub fn append_line(mut self, line: &str) -> Self {
        self.lines.extend(
            line
                .lines()
                .map(|x| x.trim().to_string())
                .collect::<Vec<String>>(),
        );
        self
    }
    pub fn insert_line(mut self, index: usize, line: &str) -> Self {
        self.lines.insert(index, line.to_owned());
        self
    }
    pub fn get_line(&self, index: usize) -> String {
        self.lines.get(index).unwrap().clone()
    }
    pub fn get_range_lines(&self, range: Range<usize>) -> String {
        Vec::from_iter(self.lines[range].iter().cloned())
            .into_iter()
            .map(|x| x + "\n")
            .collect::<Vec<String>>()
            .join("")
    }
    pub fn count_lines(&self) -> usize {
        self.lines.len()
    }
    pub fn print_line(self, index: usize) -> Self {
        println!(
            "{}",
            self.lines.get(index).unwrap_or(&"Not Exist".to_owned())
        );
        self
    }
    pub fn print_all_lines(self) -> Self {
        for (i, v) in self.lines.clone().iter().enumerate() {
            println!("({})- {}", i.to_string().green(), v);
        }
        self
    }
    pub fn flush(self) -> std::io::Result<()> {
        self.file.write(self.lines)
    }
}
