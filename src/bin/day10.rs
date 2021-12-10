use advent::{get_my_lines, iter_lines};
use anyhow::{bail, Result};
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

fn main() -> Result<()> {
    let mut acc: u32 = 0;
    for line in get_my_lines!() {
        let result = NavigationParser::parse(Rule::line, &line);
        if let Err(e) = result {
            if let Pos(n) = e.location {
                // If the error position is the last character,
                // it's an incomplete pattern rather than corrupt
                if n < line.len() {
                    acc += score(line.as_bytes()[n] as char)?;
                }
            }
        }
    }
    println!("Error score: {}", acc);
    Ok(())
}
