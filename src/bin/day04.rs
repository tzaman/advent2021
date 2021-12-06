use advent::{get_my_lines, iter_lines};
use anyhow::{bail, ensure, Context, Result};
use std::num::ParseIntError;

#[derive(Debug, Default)]
struct Bingo {
    board: [[u8; 5]; 5],
    seen: [[bool; 5]; 5],
}

impl Bingo {
    fn new(it: &mut impl Iterator<Item = String>) -> Result<Self> {
        let mut b = Self::default();
        ensure!(
            it.next().context("Expected line!")? == "",
            "Expected blank line!"
        );
        for i in 0..5 {
            let mut line = it.next().context("Expected line!")?;
            let nums = line
                .split_whitespace()
                .map(|i| i.parse::<u8>())
                .collect::<Result<Vec<u8>, ParseIntError>>()
                .context("Failed to parse board!")?;

            for j in 0..5 {
                b.board[i][j] = nums[j];
            }
        }
        Ok(b)
    }
}

fn main() {
    println!("Hello day4");
    let mut lines = get_my_lines!();
    lines.next();
    let b = Bingo::new(&mut lines).unwrap();
    dbg!(b);
}
