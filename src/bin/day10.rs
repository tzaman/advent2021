use advent::{get_my_lines, iter_lines};
use anyhow::{bail, Result};
use pest::error::InputLocation::Pos;
use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "grammar/day10.pest"]
pub struct NavigationParser;

enum ParseResult {
    Success,
    Incomplete,
    Corrupted(usize),
}

fn try_parse(s: &str) -> ParseResult {
    let result = NavigationParser::parse(Rule::line, s);
    match result {
        Ok(_) => ParseResult::Success,
        Err(e) => match e.location {
            Pos(n) => {
                if n < s.len() {
                    ParseResult::Corrupted(n)
                } else {
                    ParseResult::Incomplete
                }
            }
            _ => unreachable!(),
        },
    }
}

fn err_score(c: char) -> Result<usize> {
    Ok(match c {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        _ => bail!("Unexpected character!"),
    })
}

fn autocomplete(line: &str) -> Result<usize> {
    let mut line = line.to_string();
    let mut score = 0;
    let terminals = vec![')', ']', '}', '>'];
    loop {
        for (idx, term) in terminals.iter().enumerate() {
            let attempt = format!("{}{}", &line, &term);
            match try_parse(&attempt) {
                ParseResult::Incomplete => {
                    score = score * 5 + idx + 1;
                    line = attempt;
                }
                ParseResult::Success => {
                    score = score * 5 + idx + 1;
                    return Ok(score);
                }
                ParseResult::Corrupted(_) => {}
            }
        }
    }
}

fn main() -> Result<()> {
    let mut error_score: usize = 0;
    let mut completions: Vec<usize> = Vec::new();
    for line in get_my_lines!() {
        match try_parse(&line) {
            ParseResult::Corrupted(n) => error_score += err_score(line.as_bytes()[n] as char)?,
            ParseResult::Incomplete => completions.push(autocomplete(&line)?),
            ParseResult::Success => println!("No parsing error on this line"),
        };
    }
    completions.sort_unstable();
    let comp_score = completions[(completions.len() - 1) / 2];
    println!("Error score: {}", error_score);
    println!("Completion score: {}", comp_score);
    Ok(())
}
