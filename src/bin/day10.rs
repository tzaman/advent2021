use advent::{get_my_lines, iter_lines};
use anyhow::{bail, Context, Result};
use pest::error::InputLocation::Pos;
use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "grammar/day10.pest"]
pub struct NavigationParser;

fn score(c: char) -> Result<u32> {
    Ok(match c {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        _ => bail!("Unexpected character!"),
    })
}

fn get_location(e: pest::error::Error<Rule>) -> Option<usize> {
    match e.location {
        Pos(n) => Some(n),
        _ => None,
    }
}

fn main() -> Result<()> {
    let mut error_score: u32 = 0;
    for line in get_my_lines!() {
        let result = NavigationParser::parse(Rule::line, &line);
        let n = result
            .err()
            .and_then(get_location)
            .context("Wrong error type?")?;
        if n < line.len() {
            error_score += score(line.as_bytes()[n] as char)?;
        }
    }
    println!("Error score: {}", error_score);
    Ok(())
}
