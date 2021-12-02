use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn iter_lines(filename: &str) -> impl Iterator<Item = String> {
    let file = File::open(filename).expect("File not found!");
    let reader = BufReader::new(file);
    reader.lines().map(|line| line.unwrap())
}

#[macro_export]
macro_rules! get_my_lines {
    () => {
        iter_lines(&file!().replace("bin", "input").replace(".rs", ".txt"))
    };
}

#[derive(Debug, Clone)]
pub struct InputError;

impl From<regex::Error> for InputError {
    fn from(_: regex::Error) -> InputError {
        InputError
    }
}

impl From<std::num::ParseIntError> for InputError {
    fn from(_: std::num::ParseIntError) -> InputError {
        InputError
    }
}
