use std::collections::VecDeque;
use std::fs::File;
use std::io::{BufRead, BufReader};

struct Window<const SIZE: usize> {
    values: VecDeque<u32>,
}

impl<const SIZE: usize> Window<SIZE> {
    fn push_and_check(&mut self, value: u32) -> u32 {
        if self.values.len() < SIZE {
            self.values.push_back(value);
            0
        } else {
            let prev_sum: u32 = self.values.iter().sum();
            self.values.pop_front();
            self.values.push_back(value);
            let new_sum: u32 = self.values.iter().sum();
            if new_sum > prev_sum {
                1
            } else {
                0
            }
        }
    }
}

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

fn count_increases_windowed(depths: Vec<u32>) -> u32 {
    let mut window = Window::<3> {
        values: VecDeque::new(),
    };
    let mut increasing = 0;
    for depth in depths {
        increasing += window.push_and_check(depth);
    }
    increasing
}

fn main() {
    println!("Hello, world!");

    let depths = read_input("src/input/day01/input.txt");
    let inc = count_increases(depths);
    println!("Increases: {}", inc);

    let depths = read_input("src/input/day01/input.txt");
    let inc = count_increases_windowed(depths);
    println!("Windowed increases: {}", inc);
}
