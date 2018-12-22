#[macro_use]
extern crate lazy_static;
use clap::{App, Arg};
use colored::*;
use regex::Regex;
use std::env;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::io::SeekFrom;
use std::io::{stdin, BufReader, Read};
use std::ops::Range;
use std::path::{Path, PathBuf};

#[cfg(test)]
mod test;

lazy_static! {
    static ref RE: Regex = Regex::new(r"((\d+)(\.{2})(\d*))").unwrap();
}

fn main() {
    let matches = App::new("note-rs")
        .version("0.1.0")
        .author("Geekoff <geekoffmail@gmail.com>")
        .about("note in command line")
        .arg(
            Arg::with_name("read")
                .short("r")
                .long("read")
                .value_name("line number")
                .help("Sets a line number")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("write")
                .short("w")
                .long("write")
                .value_name("string")
                .help("Sets a string line")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("delete")
                .short("d")
                .long("delete")
                .value_name("line number")
                .help("Sets a line number")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("stdin")
                .short("s")
                .long("stdin")
                .help("pip output in to note"),
        )
        .arg(
            Arg::with_name("append")
                .value_name("append string")
                .help("Sets a string line")
                .takes_value(true),
        )
        .get_matches();
    let mut file: File = open_file(note_path_file()).unwrap();
    if let Some(r) = matches.value_of("read") {
        let max = count_line(file.try_clone().unwrap()) as i32;
        for i in parse_str(max, r).unwrap() {
            print!("{}", read_line(&mut file, i).expect("not exist !"))
        }
    } else if let Some(w) = matches.value_of("write") {
        write_line(&mut file, w);
    } else if let Some(d) = matches.value_of("delete") {
        let max = count_line(file.try_clone().unwrap()) as i32;
        del_line(&mut file, parse_str(max, d).expect("parse error !"));
    } else if let Some(a) = matches.value_of("append") {
        write_line(&mut file, a.trim());
    } else if matches.is_present("stdin") {
        let mut buf = String::new();
        stdin().read_to_string(&mut buf).unwrap();
        write_line(&mut file, buf.trim());
    } else {
        if let None = env::args().nth(1) {
            print_all_file(file);
        }
    }
}
fn note_path_file() -> PathBuf {
    dirs::home_dir().unwrap().join(Path::new("note"))
}
fn del_line(file: &mut File, line: Range<i32>) {
    let bf: BufReader<File> = BufReader::new(file.try_clone().unwrap());
    let v: Vec<String> = bf.lines().map(|x| x.unwrap()).collect();
    let mut out: Vec<String> = Vec::new();
    let not: Vec<i32> = line.map(|x| x - 1).collect();
    for (i, s) in v.iter().enumerate() {
        if !not.contains(&(i as i32)) {
            out.push(s.clone());
        }
    }
    file.set_len(0).unwrap();
    file.seek(SeekFrom::Start(0)).unwrap();
    for l in out.iter() {
        writeln!(file, "{}", l).unwrap();
    }
    file.flush().unwrap();
    file.seek(SeekFrom::Start(0)).unwrap();
}

fn parse_str(max_line: i32, s: &str) -> Option<Range<i32>> {
    if RE.is_match(s) {
        let mut num = String::new();
        let mut start = 0;
        let stop;
        let mut b = 0;
        for c in s.chars() {
            if c.is_numeric() {
                num.push(c);
            } else if c == '.' {
                b += 1;
                if b == 2 {
                    start = num.parse().unwrap();
                    num.clear()
                }
            }
        }
        if let Ok(x) = num.parse::<i32>() {
            stop = x;
        } else {
            if b == 2 {
                stop = max_line;
            } else {
                panic!("input invalid !");
            }
        }
        if start <= stop && b == 2 {
            return Some(start..stop + 1);
        } else {
            println!("not exist line");
            return None;
        }
    } else if let Ok(i) = s.parse::<i32>() {
        return Some(i..i + 1);
    } else {
        return None;
    }
}

fn read_line(file: &mut File, line: i32) -> Option<String> {
    let mut bf: BufReader<File> = BufReader::new(file.try_clone().unwrap());
    for i in 1.. {
        let mut out = String::new();
        match bf.read_line(&mut out) {
            Ok(x) => {
                if x == 0 {
                    break;
                } else {
                    if i == line {
                        file.seek(SeekFrom::Start(0)).unwrap();
                        return Some(out);
                    }
                }
            }
            Err(_) => break,
        }
    }
    file.seek(SeekFrom::Start(0)).unwrap();
    None
}

pub fn print_all_file(file: File) {
    let mut bf: BufReader<File> = BufReader::new(file);
    for i in 1.. {
        let mut out = String::new();
        match bf.read_line(&mut out) {
            Ok(x) => {
                if x == 0 {
                    break;
                } else {
                    print!("({})- {}", i.to_string().green(), out);
                }
            }
            Err(_) => break,
        }
    }
}

pub fn open_file<T: AsRef<Path>>(path: T) -> std::io::Result<File> {
    OpenOptions::new()
        .write(true)
        .create(true)
        .append(true)
        .read(true)
        .open(path)
}

pub fn write_line(file: &mut File, ss: &str) {
    writeln!(file, "{}", ss).unwrap();
}

fn count_line(file: File) -> usize {
    let f: BufReader<File> = BufReader::new(file);
    f.lines().count()
}
