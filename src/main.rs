#[macro_use]
extern crate clap;

mod cli;
use cli::*;
use note_rs::*;
use std::io::{self, BufRead};
use std::path::{Path, PathBuf};
use std::process::exit;

fn main() {
    let note = Notes::new(note_file_path());
    match AppArgs::get() {
        AppArgs::Read(i) => {
            match i {
                Input::Number(n) => {
                    println!("{}", note.get_line(n));
                }
                Input::RangeNumber(r) => {
                    println!("{}", note.get_range_lines(r));
                }
            }
            exit(0);
        }
        AppArgs::Write(w) => {
            println!("{:?}", note.append_line(&*w).flush());
            exit(0);
        }
        AppArgs::Delete(d) => {
            match d {
                Input::Number(n) => println!("{:?}", note.delete_line(n).flush()),
                Input::RangeNumber(r) => println!("{:?}", note.delete_range_lines(r).flush()),
            }
            exit(0);
        }
        AppArgs::Print => {
            note.print_all_lines();
        }
        AppArgs::None => {
            note.append_line(io::stdin().lock().lines().next().unwrap().unwrap().as_str())
                .flush()
                .unwrap();
        }
    }
}

fn note_file_path() -> PathBuf {
    dirs::home_dir().unwrap().join(Path::new(".note"))
}
