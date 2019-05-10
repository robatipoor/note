use clap::{App, Arg, ArgMatches};
use std::ops::Range;

#[derive(Debug)]
pub enum Input<T> {
    Number(T),
    RangeNumber(Range<T>),
}

#[derive(Debug)]
pub enum SubCommand {
    Delete(Input<usize>),
    Read(Input<usize>),
    Write(String),
    None,
}

impl Default for SubCommand {
    fn default() -> Self {
        SubCommand::None
    }
}

impl SubCommand {
    pub fn get() -> SubCommand {
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
            .get_matches();
        let mut sub = SubCommand::default();
        if let Some(r) = matches.value_of("read") {
            sub = SubCommand::Read(SubCommand::parse_range_str(r).unwrap());
        } else if let Some(w) = matches.value_of("write") {
            sub = SubCommand::Write(w.to_owned());
        } else if let Some(d) = matches.value_of("delete") {
            sub = SubCommand::Delete(SubCommand::parse_range_str(d).unwrap());
        }
        sub
    }

    fn parse_range_str(input: &str) -> Option<Input<usize>> {
        match input.trim().parse::<usize>() {
            Ok(o) => return Some(Input::Number(o)),
            Err(_) => {
                let v: Vec<usize> = input
                    .split("..")
                    .filter(|x| x.len() > 0)
                    .map(|x| x.trim().parse::<usize>().unwrap())
                    .collect();
                if v.len() == 2 {
                    Some(Input::RangeNumber(
                        v.get(0).unwrap().clone()..v.get(1).unwrap().clone(),
                    ))
                } else {
                    None
                }
            }
        }
    }
}
