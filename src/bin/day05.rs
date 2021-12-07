use advent::{get_my_lines, iter_lines};
use anyhow::{anyhow, Context, Error, Result};
use counter::Counter;
use itertools::Itertools;
use std::cmp::Ordering;
use std::str::FromStr;

#[derive(Clone, Debug, Default)]
struct Vent {
    start: (i32, i32),
    end: (i32, i32),
    x_step: i32,
    y_step: i32,
}

fn parse_tuple(s: &str) -> Result<(i32, i32)> {
    let (x, y) = s
        .split(',')
        .map(|i| i.parse::<i32>())
        .next_tuple()
        .with_context(|| format!("Failed to split '{}' on comma", s))?;
    Ok((x?, y?))
}

fn step(i: i32, j: i32) -> i32 {
    match i.cmp(&j) {
        Ordering::Less => 1,
        Ordering::Equal => 0,
        Ordering::Greater => -1,
    }
}

impl FromStr for Vent {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self> {
        let (first, second) = s
            .split(" -> ")
            .next_tuple()
            .ok_or_else(|| anyhow!("Failed to split '{}' on ->", s))?;
        let (start, end) = (parse_tuple(first)?, parse_tuple(second)?);
        Ok(Vent {
            start,
            end,
            x_step: step(start.0, end.0),
            y_step: step(start.1, end.1),
        })
    }
}

impl Iterator for Vent {
    type Item = (i32, i32);

    fn next(&mut self) -> Option<Self::Item> {
        if self.start.0 == self.end.0 + self.x_step && self.start.1 == self.end.1 + self.y_step {
            None
        } else {
            let pos = (self.start.0, self.start.1);
            self.start.0 += self.x_step;
            self.start.1 += self.y_step;
            Some(pos)
        }
    }
}

fn parse_input() -> Result<Vec<Vent>> {
    get_my_lines!()
        .map(|line| line.parse::<Vent>())
        .collect::<Result<Vec<Vent>>>()
}

fn solve_p1() -> Result<usize> {
    let vents = parse_input()?;
    let mut sparse_map: Counter<(i32, i32)> = Counter::new();
    for vent in vents {
        if vent.start.0 == vent.end.0 || vent.start.1 == vent.end.1 {
            for pos in vent {
                sparse_map[&pos] += 1;
            }
        }
    }
    Ok(sparse_map.values().filter(|&v| *v > 1).count())
}

fn solve_p2() -> Result<usize> {
    let vents = parse_input()?;
    let mut sparse_map: Counter<(i32, i32)> = Counter::new();
    for vent in vents {
        for pos in vent {
            sparse_map[&pos] += 1;
        }
    }
    Ok(sparse_map.values().filter(|&v| *v > 1).count())
}

fn main() -> Result<()> {
    println!("Dangerous vents: {}", solve_p1()?);
    println!("Dangerous vents (diagonal): {}", solve_p2()?);
    Ok(())
}
