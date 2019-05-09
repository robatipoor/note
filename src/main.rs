#[macro_use]
extern crate clap;

mod cli;
use cli::*;
use note_rs::*;
use std::path::{Path, PathBuf};

fn main() {
    let note = Notes::new(note_file());
    match SubCommand::get() {
        SubCommand::Read(r) => {
        }
        SubCommand::Write(w) => {
        }
        SubCommand::Delete(d) => {}
        SubCommand::None => {}
    }
}

fn note_file() -> PathBuf {
    dirs::home_dir().unwrap().join(Path::new(".note"))
}
