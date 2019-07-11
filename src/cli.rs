use clap::{App, Arg, ArgMatches};
use std::ops::Range;

#[derive(Debug)]
pub enum Input<T> {
    Number(T),
    RangeNumber(Range<T>),
}

#[derive(Debug)]
pub enum AppArgs {
    Delete(Input<usize>),
    Read(Input<usize>),
    Write(String),
    Print,
    None,
}

impl Default for AppArgs {
    fn default() -> Self {
        AppArgs::None
    }
}

impl AppArgs {
    pub fn get() -> AppArgs {
        let matches: ArgMatches = App::new(crate_name!())
            .version(crate_version!())
            .author(crate_authors!())
            .about(crate_description!())
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
                Arg::with_name("print")
                    .short("p")
                    .long("print")
                    .help("Print all notes")
                    .takes_value(false),
            )
            .get_matches();
        let mut sub = AppArgs::default();
        if let Some(r) = matches.value_of("read") {
            sub = AppArgs::Read(AppArgs::parse_range_str(r).unwrap());
        } else if let Some(w) = matches.value_of("write") {
            sub = AppArgs::Write(w.to_owned());
        } else if let Some(d) = matches.value_of("delete") {
            sub = AppArgs::Delete(AppArgs::parse_range_str(d).unwrap());
        } else if matches.is_present("print") {
            sub = AppArgs::Print;
        }
        sub
    }

    fn parse_range_str(pattern: &str) -> Option<Input<usize>> {
        if pattern.contains("..") {
            let eq_sign = pattern.contains('=');
            let mut pattern = String::from(pattern);
            if eq_sign {
                pattern.remove(pattern.find('=').unwrap());
            }
            let s: Vec<&str> = pattern
                .split("..")
                .filter(|x| x != &"" )
                .collect::<Vec<&str>>();
            if s.len() == 2 {
                let start: usize = s[0].parse().unwrap();
                let mut end: usize = s[1].parse().unwrap();
                if eq_sign {
                    end += 1;
                }
                Some(Input::RangeNumber(Range { start, end }))
            } else {
                None
            }
        } else {
            Some(Input::Number(pattern.parse().unwrap()))
        }
    }
}
