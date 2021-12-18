use advent::{get_my_lines, iter_lines};
use anyhow::{Context, Result};
use counter::Counter;
use itertools::Itertools;
use std::collections::HashMap;

type Rules = HashMap<(char, char), char>;

fn parse_input() -> Result<(String, Rules)> {
    let mut rules = HashMap::new();
    let mut lines = get_my_lines!();
    let template = lines.next().context("Expected template line.")?;
    lines.next().context("Expected blank line.")?;
    for line in lines {
        let (src, dst) = line
            .split(" -> ")
            .next_tuple()
            .context("Failed to split on ' -> '")?;
        let src = src
            .chars()
            .next_tuple()
            .context("Expected 2 characters for rule source")?;
        let dst = dst
            .chars()
            .next()
            .context("Expected 1 character for rule target")?;
        rules.insert(src, dst);
    }
    Ok((template, rules))
}

fn pair_insertion(pairs: &Counter<(char, char)>, rules: &Rules) -> Result<Counter<(char, char)>> {
    let mut new_counts = Counter::new();
    for (pair, freq) in pairs.iter() {
        let insert = rules
            .get(pair)
            .with_context(|| format!("No rule found for {:?}", pair))?;
        new_counts[&(pair.0, *insert)] += freq;
        new_counts[&(*insert, pair.1)] += freq;
    }
    Ok(new_counts)
}

fn freq_diff(pairs: &Counter<(char, char)>) -> Result<usize> {
    let mut letter_freqs = Counter::<char>::new();
    for (pair, freq) in pairs.iter() {
        letter_freqs[&pair.0] += freq;
        letter_freqs[&pair.1] += freq;
    }
    for (letter, freq) in letter_freqs.clone().iter() {
        // Correct for double-counting interior letters
        // by halving all frequencies and rounding up
        letter_freqs[letter] = (freq % 2) + (freq / 2);
    }
    let counts = letter_freqs.most_common_ordered();
    let most = counts.first().context("Empty frequency counter!")?.1;
    let least = counts.last().context("Empty frequency counter!")?.1;
    Ok(most - least)
}

fn solve(template: &str, rules: &Rules, times: usize) -> Result<usize> {
    let mut pair_freqs = template
        .chars()
        .tuple_windows::<(_, _)>()
        .collect::<Counter<_>>();
    for _ in 0..times {
        pair_freqs = pair_insertion(&pair_freqs, rules)?;
    }
    freq_diff(&pair_freqs)
}

fn main() -> Result<()> {
    let (template, rules) = parse_input()?;
    println!("After 10 steps: {}", solve(&template, &rules, 10)?);
    println!("After 40 steps: {}", solve(&template, &rules, 40)?);
    Ok(())
}
