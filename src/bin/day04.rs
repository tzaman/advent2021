use advent::{get_my_lines, iter_lines};
use anyhow::{anyhow, bail, ensure, Context, Result};
use std::num::ParseIntError;

#[derive(Debug, Default)]
struct Bingo {
    board: [[u8; 5]; 5],
    seen: [[bool; 5]; 5],
}

impl Bingo {
    fn new(it: &mut impl Iterator<Item = String>) -> Result<Option<Self>> {
        let mut b = Self::default();
        match it.next() {
            Some(s) => ensure!(s == "", format!("Expected blank line, found: '{}'!", s)),
            None => return Ok(None),
        }
        for i in 0..5 {
            let line = it
                .next()
                .with_context(|| format!("Expected line {} of board!", i + 1))?;
            let nums = line
                .split_whitespace()
                .map(|i| i.parse::<u8>())
                .collect::<Result<Vec<u8>, ParseIntError>>()
                .with_context(|| format!("Failed to parse board at line: '{}'!", line))?;

            for j in 0..5 {
                b.board[i][j] = nums[j];
            }
        }
        Ok(Some(b))
    }

    fn mark(&mut self, number: u8) -> Option<usize> {
        for i in 0..5 {
            for j in 0..5 {
                if self.board[i][j] == number {
                    self.seen[i][j] = true;
                    return self.check();
                }
            }
        }
        None
    }

    fn check(&self) -> Option<usize> {
        for i in 0..5 {
            if (0..5)
                .map(|j| self.seen[i][j])
                .reduce(|acc, elt| acc & elt)?
            {
                return Some(self.unmarked_sum());
            }
        }
        for j in 0..5 {
            if (0..5)
                .map(|i| self.seen[i][j])
                .reduce(|acc, elt| acc & elt)?
            {
                return Some(self.unmarked_sum());
            }
        }
        None
    }

    fn unmarked_sum(&self) -> usize {
        let board = self.board.iter().flatten();
        let seen = self.seen.iter().flatten();
        Iterator::zip(board, seen)
            .filter(|(_, &marked)| !marked)
            .map(|(&num, _)| num as usize)
            .sum()
    }
}

fn parse_numbers(it: &mut impl Iterator<Item = String>) -> Result<Vec<u8>> {
    it.next()
        .ok_or(anyhow!("Bingo numbers not found!"))?
        .split(",")
        .map(|i| i.parse::<u8>())
        .collect::<Result<Vec<u8>, ParseIntError>>()
        .context("Failed to parse bingo numbers!")
}

fn parse_input() -> Result<(Vec<u8>, Vec<Bingo>)> {
    let mut lines = get_my_lines!();
    let mut boards: Vec<Bingo> = Vec::new();
    let numbers = parse_numbers(&mut lines)?;
    loop {
        let board = Bingo::new(&mut lines)?;
        match board {
            Some(board) => boards.push(board),
            None => break,
        }
    }
    Ok((numbers, boards))
}

fn solve_p1() -> Result<usize> {
    let (numbers, mut boards) = parse_input()?;
    for num in numbers {
        for board in boards.iter_mut() {
            if let Some(value) = board.mark(num) {
                return Ok(value * num as usize);
            }
        }
    }
    bail!("No winning board found!")
}

fn main() -> Result<()> {
    println!("Winning board value: {}", solve_p1()?);
    Ok(())
}
