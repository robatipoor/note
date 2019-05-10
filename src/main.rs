#[macro_use]
extern crate clap;

mod cli;
use cli::*;
use note_rs::*;
// use std::io::{self, BufRead};
use std::path::{Path, PathBuf};

fn main() {
    let note = Notes::new(note_file());
    match SubCommand::get() {
        SubCommand::Read(i) => match i {
            Input::Number(n) => println!("{}", note.get_line(n)),
            Input::RangeNumber(r) => println!("{}", note.get_range_lines(r)),
        },
        SubCommand::Write(w) => {
            println!("{:?}", note.append_line(&*w).flush());
        }
        SubCommand::Delete(d) => match d {
            Input::Number(n) => println!("{:?}", note.delete_line(n).flush()),
            Input::RangeNumber(r) => println!("{:?}", note.delete_range_lines(r).flush()),
        },
        SubCommand::None => {
            note.print_all_lines();
        }
    }
    // todo!();
    // let content = io::stdin().lock().lines().next().map(|x| match x {
    //     Ok(c) => c,
    //     Err(e) => panic!("[error] unable to read piped text: {}", e),
    // });
}

fn note_file() -> PathBuf {
    dirs::home_dir().unwrap().join(Path::new(".note"))
}
