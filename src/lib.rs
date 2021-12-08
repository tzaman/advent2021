use anyhow::{Context, Result};
use std::error::Error;
use std::fmt;
use std::fs::{read_to_string, File};
use std::io::{BufRead, BufReader};

pub fn iter_lines(filename: &str) -> impl Iterator<Item = String> {
    let file = File::open(filename).expect("File not found!");
    let reader = BufReader::new(file);
    reader.lines().map(|line| line.unwrap())
}

pub fn iter_csv_line(filename: &str) -> Result<Vec<String>> {
    Ok(read_to_string(&filename)
        .with_context(|| format!("Couldn't read input from {}", filename))?
        .split(',')
        .map(String::from)
        .collect())
}

#[macro_export]
macro_rules! get_my_lines {
    () => {
        iter_lines(&file!().replace("bin", "input").replace(".rs", ".txt"))
    };
}

#[macro_export]
macro_rules! get_my_values {
    () => {
        iter_csv_line(&file!().replace("bin", "input").replace(".rs", ".txt"))
    };
}

#[derive(Debug, Clone)]
pub struct InputError;

impl fmt::Display for InputError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Failed to parse input!")
    }
}

impl Error for InputError {}

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
