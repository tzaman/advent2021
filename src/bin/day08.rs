use advent::{get_my_lines, iter_lines};
use anyhow::{Context, Error, Result};
use itertools::Itertools;
use std::collections::HashSet;
use std::str::FromStr;

// 2 segments = 1 |..c..f.|
// 3 segments = 7 |a.c..f.|
// 4 segments = 4 |.bcd.f.|
// 7 segments = 8 |abcdefg|

// 5 segments = 2 |a.cde.g| => #.c#e.#
// 5 segments = 3 |a.cd.fg| => #.c#.f# (matches 1)
// 5 segments = 5 |ab.d.fg| => #b.#.f# (subset of 4)

// 6 segments = 0 |abc.efg| => #b#.e##
// 6 segments = 6 |a.cdefg| => #.#de##
// 6 segments = 9 |abcd.fg| => #b#d.## (subset of 4)

#[derive(Debug, Default)]
struct Signal {
    patterns: Vec<String>,
    value: Vec<String>,
}

impl FromStr for Signal {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let (p, v) = s.split(" | ").next_tuple().context("Failed to parse")?;
        Ok(Signal {
            patterns: p.split_whitespace().map(String::from).collect(),
            value: v.split_whitespace().map(String::from).collect(),
        })
    }
}

fn parse_input() -> Result<Vec<Signal>> {
    get_my_lines!()
        .map(|line| line.parse::<Signal>())
        .collect::<Result<Vec<Signal>>>()
}

fn solve_p1() -> Result<usize> {
    let signals = parse_input()?;
    let unique_lengths: HashSet<_> = vec![2, 3, 4, 7].into_iter().collect();
    Ok(signals
        .iter()
        .flat_map(|s| s.value.iter().filter(|v| unique_lengths.contains(&v.len())))
        .count())
}

fn main() -> Result<()> {
    println!("Number of unique digits: {}", solve_p1()?);
    Ok(())
}
