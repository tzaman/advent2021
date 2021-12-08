use advent::{get_my_values, iter_csv_line};
use anyhow::{Context, Result};
use std::convert::identity;

fn parse_input() -> Result<Vec<i64>> {
    get_my_values!()?
        .iter()
        .map(|v| {
            v.parse::<i64>()
                .with_context(|| format!("Failed to parse {}", v))
        })
        .collect()
}

fn arithmetic_sum(n: i64) -> i64 {
    n * (n + 1) / 2
}

fn fuel_to_reach(crabs: &[i64], pos: i64, cost: fn(i64) -> i64) -> i64 {
    crabs.iter().map(|c| cost((pos - c).abs())).sum()
}

fn solve(cost: fn(i64) -> i64) -> Result<i64> {
    let crabs = parse_input()?;
    let max = *crabs.iter().max().context("No crabs!")?;
    (0..max)
        .map(|pos| fuel_to_reach(&crabs, pos, cost))
        .min()
        .context("No minimum?")
}

fn main() -> Result<()> {
    println!("Fuel cost: {}", solve(identity)?);
    println!("Real cost: {}", solve(arithmetic_sum)?);
    Ok(())
}
