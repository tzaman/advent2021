use advent::{get_my_lines, iter_lines};
use regex::Regex;

#[derive(Debug)]
struct Position{
    x: u32,
    y: u32,
    aim: u32,
}

impl Position{
    fn new() -> Position {
        Position{ x: 0, y: 0, aim: 0}
    }

    fn update(&mut self, command: &str) {
        let re = Regex::new(r"(forward|up|down) (\d+)").unwrap();
        let cap = re.captures(command).expect("Failed to parse line.");
        let direction = cap.get(1).unwrap().as_str();
        let units = cap.get(2).unwrap().as_str().parse::<u32>().unwrap();
        match direction {
            "forward" => self.x += units,
            "up" => self.y -= units,
            "down" => self.y += units,
            _ => panic!("Unknown direction."),
        }
    }

    fn update_better(&mut self, command: &str) {
        let re = Regex::new(r"(forward|up|down) (\d+)").unwrap();
        let cap = re.captures(command).expect("Failed to parse line.");
        let direction = cap.get(1).unwrap().as_str();
        let units = cap.get(2).unwrap().as_str().parse::<u32>().unwrap();
        match direction {
            "forward" => {
                self.x += units;
                self.y += units * self.aim;
            },
            "up" => self.aim -= units,
            "down" => self.aim += units,
            _ => panic!(),
        }
    }
}

fn follow_directions() -> u32 {
    let mut pos = Position::new();
    get_my_lines!().for_each(
        |line| pos.update(&line)
    );
    pos.x * pos.y
}

fn follow_directions_better() -> u32 {
    let mut pos = Position::new();
    get_my_lines!().for_each(
        |line| pos.update_better(&line)
    );
    pos.x * pos.y
}

fn main() {
    println!("Final multiple: {}", follow_directions());
    println!("Better multiple: {}", follow_directions_better());
}
