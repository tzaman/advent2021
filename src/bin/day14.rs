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

fn pair_insertion(polymer: &str, rules: &Rules) -> Result<String> {
    let mut expand = String::new();
    for pair in polymer.chars().collect_vec().windows(2) {
        expand.push(pair[0]);
        expand.push(rules[&(pair[0], pair[1])]);
    }
    expand.push(polymer.chars().last().context("Empty polymer!")?);
    Ok(expand)
}

fn freq_diff(s: &str) -> Result<usize> {
    let counts = s.chars().collect::<Counter<_>>().most_common_ordered();
    let most = counts[0].1;
    let least = counts.last().context("Empty frequency vector!")?.1;
    Ok(most - least)
}

fn solve_p1(template: &str, rules: &Rules) -> Result<usize> {
    let mut temp = template.to_string();
    for _ in 0..10 {
        temp = pair_insertion(&temp, rules)?;
    }
    freq_diff(&temp)
}

fn main() -> Result<()> {
    let (template, rules) = parse_input()?;
    println!("Most - least common: {}", solve_p1(&template, &rules)?);
    Ok(())
}
