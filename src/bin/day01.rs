use std::fs::File;
use std::io::{BufRead, BufReader};

fn read_input(filename: &str) -> Vec<u32> {
    let file = File::open(filename).expect("File not found!");
    let reader = BufReader::new(file);
    reader
        .lines()
        .map(|line| line.unwrap().parse::<u32>().unwrap())
        .collect()
}

fn count_increases(depths: Vec<u32>) -> u32 {
    let mut prev: Option<u32> = None;
    let mut increasing = 0;

    for depth in depths {
        if let Some(p) = prev {
            if depth > p {
                increasing += 1;
            }
        }
        prev = Some(depth);
    }
    increasing
}

fn main() {
    println!("Hello, world!");

    let depths = read_input("src/input/day01/input.txt");
    let inc = count_increases(depths);

    println!("Increases: {}", inc);
}
