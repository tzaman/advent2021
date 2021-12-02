use advent::{get_my_lines, iter_lines};
use std::collections::VecDeque;

struct Window<const SIZE: usize> {
    values: VecDeque<u32>,
}

impl<const SIZE: usize> Window<SIZE> {
    fn new() -> Window<SIZE> {
        Window {
            values: VecDeque::new(),
        }
    }

    fn push_and_check(&mut self, value: u32) -> u32 {
        let prev_sum: u32 = self.values.iter().sum();
        self.values.push_back(value);
        if self.values.len() <= SIZE {
            return 0;
        }
        self.values.pop_front();
        let new_sum: u32 = self.values.iter().sum();
        (new_sum > prev_sum) as u32
    }
}

fn parse_input() -> Vec<u32> {
    get_my_lines!()
        .map(|line| line.parse::<u32>().unwrap())
        .collect()
}

fn windowed_sweep<const SIZE: usize>(depths: Vec<u32>) -> u32 {
    let mut window = Window::<SIZE>::new();
    depths
        .iter()
        .map(|&depth| window.push_and_check(depth))
        .sum()
}

fn main() {
    println!("Depth increases: {}", windowed_sweep::<1>(parse_input()));
    println!("Windowed increases: {}", windowed_sweep::<3>(parse_input()));
}
