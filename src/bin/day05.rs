use advent::{get_my_lines, iter_lines};
use anyhow::{anyhow, Context, Error, Result};
use counter::Counter;
use itertools::Itertools;
use std::cmp::{max, min};
use std::str::FromStr;

#[derive(Clone, Debug, Default)]
struct Vent {
    start: (u32, u32),
    end: (u32, u32),
}

fn parse_tuple(s: &str) -> Result<(u32, u32)> {
    let (x, y) = s
        .split(',')
        .map(|i| i.parse::<u32>())
        .next_tuple()
        .with_context(|| format!("Failed to split '{}' on comma", s))?;
    Ok((x?, y?))
}

impl FromStr for Vent {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self> {
        let (first, second) = s
            .split(" -> ")
            .next_tuple()
            .ok_or_else(|| anyhow!("Failed to split '{}' on ->", s))?;
        Ok(Vent {
            start: parse_tuple(first)?,
            end: parse_tuple(second)?,
        })
    }
}

fn parse_input() -> Result<Vec<Vent>> {
    get_my_lines!()
        .map(|line| line.parse::<Vent>())
        .collect::<Result<Vec<Vent>>>()
}

fn solve_p1() -> Result<usize> {
    let vents = parse_input()?;
    // dbg!(&vents);
    let mut sparse_map: Counter<(u32, u32)> = Counter::new();
    for vent in vents {
        if vent.start.0 == vent.end.0 {
            let low = min(vent.start.1, vent.end.1);
            let high = max(vent.start.1, vent.end.1) + 1;
            for j in low..high {
                sparse_map[&(vent.start.0, j)] += 1;
            }
        }
        if vent.start.1 == vent.end.1 {
            let low = min(vent.start.0, vent.end.0);
            let high = max(vent.start.0, vent.end.0) + 1;
            for i in low..high {
                sparse_map[&(i, vent.start.1)] += 1;
            }
        }
    }
    Ok(sparse_map.values().filter(|&v| *v > 1).count())
}

fn main() -> Result<()> {
    println!("Dangerous vents: {}", solve_p1()?);
    Ok(())
}
