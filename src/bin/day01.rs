use std::fs::File;
use std::io::{BufRead, BufReader};

fn read_input(filename: &str) -> Vec<i32> {
    let file = File::open(filename).expect("File not found!");
    let reader = BufReader::new(file);
    reader
        .lines()
        .map(|line| line.unwrap().parse::<i32>().unwrap())
        .collect()
}

fn main() {
    println!("Hello, world!");

    let depths = read_input("src/input/day01/input.txt");

    let mut prev = -1;
    let mut increasing = 0;

    for depth in depths {
        if prev != -1 && depth > prev {
            increasing += 1;
        }
        prev = depth;
    }
    println!("Increases: {}", increasing);
}
