use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

pub fn manually_read_from_file() {
    // crashes if README.md not exist
    let f = File::open("README.md").unwrap();
    let mut reader = BufReader::new(f);

    let mut line = String::new();

    loop {
        // len would always be 1 byte more then the .lines() method
        let len = reader.read_line(&mut line).unwrap();
        if len == 0 {
            break;
        }

        println!("{} ({} bytes long)", line, len);

        line.truncate(0); // shrink String back to length 0
    }
}

pub fn read_from_file() {
    // crashes if README.md not exist
    let f = File::open("README.md").unwrap();
    let mut reader = BufReader::new(f);

    // BufReader::lines() removes trailing newline character from each line
    for line_ in reader.lines() {
        let line = line_.unwrap();
        println!("{} ({} bytes long)", line, line.len());
    }
}
