use advent::{get_my_lines, iter_lines};
use anyhow::{Context, Result};
use itertools::Itertools;
use std::num::ParseIntError;

#[derive(Debug, Default)]
struct Cavern {
    grid: Vec<Vec<u8>>,
}

impl std::fmt::Display for Cavern {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let disp = self.grid.iter().map(|row| row.iter().join("")).join("\n");
        write!(f, "{}", disp)
    }
}

impl Cavern {
    fn new(it: &mut impl Iterator<Item = String>) -> Result<Self> {
        let mut cavern = Self::default();
        for line in it {
            let numbers: Vec<u8> = line
                .chars()
                .map(|c| c.to_string().parse::<u8>())
                .collect::<Result<Vec<u8>, ParseIntError>>()
                .context("Failed to parse line")?;
            cavern.grid.push(numbers);
        }
        Ok(cavern)
    }

    fn neighbors(&self, pos: (usize, usize)) -> Vec<(usize, usize)> {
        let mut nbrs = Vec::new();
        if pos.0 > 0 {
            nbrs.push((pos.0 - 1, pos.1));
        }
        if pos.0 < self.grid.len() - 2 {
            nbrs.push((pos.0 + 1, pos.1));
        }
        if pos.1 > 0 {
            nbrs.push((pos.0, pos.1 - 1));
        }
        if pos.1 < self.grid[0].len() - 2 {
            nbrs.push((pos.0, pos.1 + 1));
        }
        nbrs
    }

    fn find_path(&self) -> usize {
        let mut risk = vec![vec![usize::MAX; self.grid[0].len()]; self.grid.len()];
        let mut changed = true;
        risk[0][0] = 0;

        while changed {
            changed = false;
            for i in 0..self.grid.len() {
                for j in 0..self.grid[i].len() {
                    for nbr in self.neighbors((i, j)) {
                        if risk[nbr.0][nbr.1] == usize::MAX {
                            continue;
                        }
                        let cost_from = self.grid[i][j] as usize + risk[nbr.0][nbr.1];
                        if risk[i][j] > cost_from {
                            risk[i][j] = cost_from;
                            changed = true;
                        }
                    }
                }
            }
        }
        *risk.last().unwrap().last().unwrap()
    }

    fn tile(&mut self, size: usize) {
        // Tiling horizontally
        for row in &mut self.grid {
            let copied = row.clone();
            for i in 1..size {
                row.extend(copied.iter().map(|n| wrapping_add(*n, i as u8)));
            }
        }
        // Tiling vertically
        let copied = self.grid.clone();
        for i in 1..size {
            for row in &copied {
                let new_row = row.iter().map(|n| wrapping_add(*n, i as u8)).collect_vec();
                self.grid.push(new_row);
            }
        }
    }
}

fn wrapping_add(num: u8, inc: u8) -> u8 {
    let mut num = num + inc;
    while num > 9 {
        num -= 9;
    }
    num
}

fn main() -> Result<()> {
    let mut cave = Cavern::new(&mut get_my_lines!())?;
    println!("Best path: {}", cave.find_path());
    cave.tile(5);
    println!("Best path with tiling: {}", cave.find_path());
    Ok(())
}
