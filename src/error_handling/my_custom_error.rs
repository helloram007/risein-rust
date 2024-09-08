use std::fmt::write;
use std::fs;
use std::io;
use std::num;

fn main() {
    match read_and_parse("test.txt") {
        Ok(num) => print!("There is nothing wrong here"),
        Err(err) => println!("An Error has occured {}", err),
    }
}

#[derive(Debug)]
enum MyCustomError {
    Io(std::io::Error),
    Parse(std::num::ParseIntError),
    Other(String),
}

use std::fmt;

impl fmt::Display for MyCustomError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            MyCustomError::Io(err) => write!(f, "/O error: {}", err),
            MyCustomError::Parse(err) => write!(f, "Parse Error: {}", err),
            MyCustomError::Other(message) => write!(f, "Other error: {}", message),
        }
    }
}

impl std::error::Error for MyCustomError {}

fn read_and_parse(filename: &str) -> Result<i32, MyCustomError> {
    let content = fs::read_to_string(filename).map_err(MyCustomError::Io)?;
    let num = content.trim().parse().map_err(MyCustomError::Parse)?;
    Ok(num)
}
