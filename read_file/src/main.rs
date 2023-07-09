//! Read files

#![allow(dead_code)]

use std::fmt;
use std::fmt::Display;

#[derive(Debug, PartialEq)]
enum FileState {
    Open,
    Closed,
}

#[derive(Debug)]
struct File {
    name: String,
    data: Vec<u8>,
    state: FileState,
}

impl Display for FileState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            FileState::Open => write!(f, "OPEN"),
            FileState::Closed => write!(f, "CLOSED"),
        }
    }
}

impl Display for File {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "<{} ({})>", self.name, self.state)
    }
}

trait Read {
    fn read(self: &Self, save_to: &mut Vec<u8>) -> Result<usize, String>;
}

impl File {
    fn new(name: &str) -> File {
        File {
            name: String::from(name),
            data: Vec::new(),
            state: FileState::Closed,
        }
    }

    /// Returns the file's name
    fn name(&self) -> String {
        self.name.clone()
    }
}

impl Read for File {
    /// Read a file
    fn read(self: &Self, save_to: &mut Vec<u8>) -> Result<usize, String> {
        if self.state != FileState::Open {
            return Err(String::from("File must be open for reading!"));
        }

        let mut tmp = self.data.clone();
        let read_length = tmp.len();
        save_to.reserve(read_length);
        save_to.append(&mut tmp);
        Ok(read_length)
    }
}

fn open(mut f: File) -> Result<File, String> {
    f.state = FileState::Open;
    Ok(f)
}

fn close(mut f: File) -> Result<File, String> {
    f.state = FileState::Closed;
    Ok(f)
}

fn main() {
    let mut f = File::new("file.txt");
    let mut buffer: Vec<u8> = vec![];

    // if f.read(&mut buffer).is_err() {
    // println!("Error reading file file.txt!");
    // }

    f = open(f).unwrap();
    let f_length = f.read(&mut buffer).unwrap();
    f = close(f).unwrap();

    let text = String::from_utf8_lossy(&buffer);

    println!("{:?}", f); // following Debug
    println!("{}", f); // following custom implementation of Display
    println!("{} is {} bytes long", &f.name(), &f_length);
    println!("{}", text);
}
