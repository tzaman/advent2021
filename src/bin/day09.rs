use advent::{get_my_lines, iter_lines};
use anyhow::{Context, Result};
use std::collections::HashSet;

fn parse_input() -> Result<Vec<Vec<u32>>> {
    get_my_lines!()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).context("Expected digit"))
                .collect::<Result<Vec<u32>>>()
        })
        .collect::<Result<Vec<Vec<u32>>>>()
}

fn neighbors(pos: (usize, usize), cave: &[Vec<u32>]) -> Vec<(usize, usize)> {
    let mut nbrs = Vec::new();
    if pos.0 > 0 {
        nbrs.push((pos.0 - 1, pos.1));
    }
    if pos.0 < cave.len() - 1 {
        nbrs.push((pos.0 + 1, pos.1));
    }
    if pos.1 > 0 {
        nbrs.push((pos.0, pos.1 - 1));
    }
    if pos.1 < cave[pos.0].len() - 1 {
        nbrs.push((pos.0, pos.1 + 1));
    }
    nbrs
}

fn flood_fill(pos: (usize, usize), cave: &[Vec<u32>]) -> usize {
    let mut seen: HashSet<(usize, usize)> = HashSet::new();
    _flood_fill(pos, cave, &mut seen);
    seen.len()
}

fn _flood_fill(pos: (usize, usize), cave: &[Vec<u32>], seen: &mut HashSet<(usize, usize)>) {
    if cave[pos.0][pos.1] == 9 || seen.contains(&pos) {
        return;
    }
    seen.insert(pos);
    neighbors(pos, cave)
        .into_iter()
        .for_each(|p| _flood_fill(p, cave, seen));
}

fn main() -> Result<()> {
    let cave = parse_input()?;
    let mut danger = 0;
    let mut basins: Vec<usize> = Vec::new();
    for i in 0..cave.len() {
        for j in 0..cave[i].len() {
            let item = cave[i][j];
            if neighbors((i, j), &cave)
                .into_iter()
                .all(|nbr| item < cave[nbr.0][nbr.1])
            {
                danger += item + 1;
                basins.push(flood_fill((i, j), &cave));
            }
        }
    }
    basins.sort_unstable();
    basins.reverse();
    let top3 = basins
        .into_iter()
        .take(3)
        .reduce(|a, b| a * b)
        .context("No elements?!")?;
    println!("Danger sum: {}", danger);
    println!("Floodfill: {}", top3);
    Ok(())
}
