use advent::{get_my_lines, iter_lines};
use anyhow::{Context, Error, Result};
use bimap::BiMap;
use itertools::Itertools;
use std::collections::HashSet;
use std::str::FromStr;

#[derive(Debug, Default)]
struct Signal {
    patterns: Vec<String>,
    value: Vec<String>,
}

fn split_signal_sequence(s: &str) -> Vec<String> {
    s.split_whitespace()
        .map(|i| i.chars().sorted().collect::<String>())
        .collect()
}

impl FromStr for Signal {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let (p, v) = s.split(" | ").next_tuple().context("Failed to parse")?;
        let patterns = split_signal_sequence(p);
        let value = split_signal_sequence(v);
        Ok(Signal { patterns, value })
    }
}

fn filter_by_length(patterns: &[String], len: usize) -> Vec<String> {
    patterns
        .iter()
        .filter(|i| i.len() == len)
        .map(String::from)
        .collect()
}

fn intersecting_chars(a: &str, b: &str) -> usize {
    let a: HashSet<_> = a.chars().collect();
    let b: HashSet<_> = b.chars().collect();
    a.intersection(&b).count()
}

/// Given a vector of strings, a comparison string, and a count:
/// extracts the (unique) string from the vector that
/// shares `count` characters with the comparison string
fn extract_match(vec: &mut Vec<String>, comp: &str, count: usize) -> Result<String> {
    let found: String = vec
        .iter()
        .find(|s| intersecting_chars(s, comp) == count)
        .with_context(|| format!("No {}-match in {:?} for {}", count, vec, comp))?
        .to_string();
    vec.retain(|s| s != &found);
    Ok(found)
}

impl Signal {
    /// Figure out the output value of a signal
    /// 1 |..c..f.| => 2 segments
    /// 4 |.bcd.f.| => 4 segments
    /// 7 |a.c..f.| => 3 segments
    /// 8 |abcdefg| => 7 segments
    ///
    /// 5 segments
    /// 3 |a.cd.fg| => intersect(1) == 2
    /// 5 |ab.d.fg| => intersect(4) == 3
    /// 2 |a.cde.g| => otherwise
    ///
    /// 6 segments
    /// 9 |abcd.fg| => intersect(4) == 4
    /// 6 |ab.defg| => intersect(5) == 5
    /// 0 |abc.efg| => otherwise
    fn output(&self) -> Result<usize> {
        let mut map: BiMap<_, _> = BiMap::new();
        map.insert(filter_by_length(&self.patterns, 2).pop().unwrap(), 1);
        map.insert(filter_by_length(&self.patterns, 4).pop().unwrap(), 4);
        map.insert(filter_by_length(&self.patterns, 3).pop().unwrap(), 7);
        map.insert(filter_by_length(&self.patterns, 7).pop().unwrap(), 8);

        let mut five_segments = filter_by_length(&self.patterns, 5);
        let three = extract_match(&mut five_segments, map.get_by_right(&1).unwrap(), 2)?;
        let five = extract_match(&mut five_segments, map.get_by_right(&4).unwrap(), 3)?;
        let two = five_segments.pop().unwrap();
        map.insert(three, 3);
        map.insert(five, 5);
        map.insert(two, 2);

        let mut six_segments = filter_by_length(&self.patterns, 6);
        let nine = extract_match(&mut six_segments, map.get_by_right(&4).unwrap(), 4)?;
        let six = extract_match(&mut six_segments, map.get_by_right(&5).unwrap(), 5)?;
        let zero = six_segments.pop().unwrap();
        map.insert(nine, 9);
        map.insert(six, 6);
        map.insert(zero, 0);

        let output = self.value.iter().fold(0, |acc, v| {
            (acc * 10) + *(map.get_by_left(v).unwrap()) as usize
        });
        Ok(output)
    }
}

fn parse_input() -> Result<Vec<Signal>> {
    get_my_lines!()
        .map(|line| line.parse::<Signal>())
        .collect::<Result<Vec<Signal>>>()
}

fn solve_p1(signals: &[Signal]) -> Result<usize> {
    let unique_lengths: HashSet<_> = vec![2, 3, 4, 7].into_iter().collect();
    Ok(signals
        .iter()
        .flat_map(|s| s.value.iter().filter(|v| unique_lengths.contains(&v.len())))
        .count())
}

fn solve_p2(signals: &[Signal]) -> Result<usize> {
    signals.iter().map(|s| s.output()).sum()
}

fn main() -> Result<()> {
    let signals = parse_input()?;
    println!("Number of unique digits: {}", solve_p1(&signals)?);
    println!("Sum of output values: {}", solve_p2(&signals)?);
    Ok(())
}
