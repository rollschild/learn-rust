use clap::{Arg, ArgAction, Command};
use regex::Regex;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::io::BufReader;

// #[derive(Parser, Debug)]
// struct Args {
// #[arg(short, long)]
// pattern: String,
//
// #[arg(short, long)]
// input: String,
// }

fn process_lines<T: BufRead + Sized>(reader: T, re: Regex) {
    for line_ in reader.lines() {
        let line = line_.unwrap(); // String
        match re.find(&line) {
            // re.find() accepts &str
            Some(_) => println!("{}", line),
            None => (),
        }
    }
}

fn main() {
    let cli = Command::new("grep-lite")
        .arg(
            Arg::new("p")
                .long("pattern")
                .required(true)
                .action(ArgAction::Set),
        )
        .arg(
            Arg::new("i")
                .long("input")
                .required(false)
                .action(ArgAction::Set),
        )
        .get_matches();

    let pattern = match cli.get_one::<String>("p") {
        Some(c) => c,
        _ => "",
    };
    let re = Regex::new(pattern).unwrap();

    // let input = cli.get_one<String>("input").unwrap_or("-");
    let input = match cli.get_one::<String>("i") {
        Some(c) => c,
        _ => "-",
    };

    if input == "-" {
        let stdin = io::stdin;
        let reader = stdin().lock();
        process_lines(reader, re);
    } else {
        let f = File::open(input).unwrap();
        let reader = BufReader::new(f);
        process_lines(reader, re);
    }
}
