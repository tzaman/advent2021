use advent::{get_my_lines, iter_lines};
use anyhow::{Context, Result};
use itertools::Itertools;
use std::num::ParseIntError;

const SIZE: usize = 10;

#[derive(Debug, Default)]
struct OctoGrid {
    board: [[u8; SIZE]; SIZE],
    flashed: [[bool; SIZE]; SIZE],
}

impl std::fmt::Display for OctoGrid {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let disp = self.board.iter().map(|row| row.iter().join(" ")).join("\n");
        write!(f, "{}", disp)
    }
}

impl OctoGrid {
    fn new(it: &mut impl Iterator<Item = String>) -> Result<Self> {
        let mut grid = Self::default();
        for i in 0..SIZE {
            let line = it
                .next()
                .with_context(|| format!("Expected line {} of board!", i + 1))?;
            let nums = line
                .chars()
                .map(|c| c.to_string().parse::<u8>())
                .collect::<Result<Vec<u8>, ParseIntError>>()
                .with_context(|| format!("Failed to parse board at line: '{}'!", line))?;
            let nums: Vec<u8> = nums.into_iter().map(|digit| digit as u8).collect();
            grid.board[i][..].clone_from_slice(&nums[..]);
        }
        Ok(grid)
    }

    fn increment(&mut self) {
        for i in 0..SIZE {
            for j in 0..SIZE {
                self.board[i][j] += 1;
                self.flashed[i][j] = false;
            }
        }
    }

    fn flash(&mut self) -> bool {
        let mut new_flash = false;
        for i in 0..SIZE {
            for j in 0..SIZE {
                if self.board[i][j] > 9 && !self.flashed[i][j] {
                    new_flash = true;
                    self.flashed[i][j] = true;
                    self.board[i][j] = 0;
                    neighbors((i, j))
                        .into_iter()
                        .filter(|(x, y)| !self.flashed[*x][*y])
                        .for_each(|(x, y)| self.board[x][y] += 1)
                }
            }
        }
        new_flash
    }

    fn step(&mut self) -> usize {
        self.increment();
        while self.flash() {} // no-op
        self.flashed
            .iter()
            .flat_map(|row| row.iter())
            .filter(|&&item| item)
            .count()
    }

    fn all_flashed(&self) -> bool {
        self.flashed
            .iter()
            .flat_map(|row| row.iter())
            .all(|&item| item)
    }
}

fn neighbors(pos: (usize, usize)) -> Vec<(usize, usize)> {
    let x_min = pos.0.saturating_sub(1);
    let x_max = std::cmp::min(pos.0 + 2, SIZE);
    let y_min = pos.1.saturating_sub(1);
    let y_max = std::cmp::min(pos.1 + 2, SIZE);
    (x_min..x_max)
        .cartesian_product(y_min..y_max)
        .filter(|&p| p != pos)
        .collect()
}

fn solve_p1() -> Result<usize> {
    let mut grid = OctoGrid::new(&mut get_my_lines!())?;
    let flashes = (0..100).map(|_| grid.step()).sum();
    println!("{}", &grid);
    Ok(flashes)
}

fn solve_p2() -> Result<usize> {
    let mut grid = OctoGrid::new(&mut get_my_lines!())?;
    let mut step = 0;
    while !grid.all_flashed() {
        grid.step();
        step += 1;
    }
    println!("{}", &grid);
    Ok(step)
}

fn main() -> Result<()> {
    println!("Flashes after 100 steps: {}\n", solve_p1()?);
    println!("Synchronized flash step: {}\n", solve_p2()?);
    Ok(())
}
