use crate::FileHandler;
use colored::*;
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

    fn delete_line(mut self, index: usize) -> Self {
        self.lines.remove(index);
        self
    }
    fn delete_range_line(mut self, range: (usize, usize)) -> Self {
        for i in range.0..range.1 {
            self.lines.remove(i);
        }
        self
    }
    fn append_lines(mut self, lines: &str) -> Self {
        self.lines.extend(
            lines
                .lines()
                .map(|x| x.trim().to_string())
                .collect::<Vec<String>>(),
        );
        self
    }
    fn insert_line(mut self, index: usize, line: &str) -> Self {
        self.lines.insert(index, line.to_owned());
        self
    }
    fn get_line(&self, index: usize) -> String {
        self.lines.get(index).unwrap().clone()
    }
    // fn get_range_lines(&self, index: usize) -> String {
    //     self.lines.get(index).unwrap().clone()
    // }
    fn count_lines(&self) -> usize {
        self.lines.len()
    }
    fn print_line(self, index: usize) -> Self {
        println!(
            "{}",
            self.lines.get(index).unwrap_or(&"Not Exist".to_owned())
        );
        self
    }
    fn print_all_lines(self) -> Self {
        for (i, v) in self.lines.clone().iter().enumerate() {
            println!("({})- {}", i.to_string().green(), v);
        }
        self
    }
    fn flush(self) -> std::io::Result<()> {
        self.file.write(self.lines)
    }
}
