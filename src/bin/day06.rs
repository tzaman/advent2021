use advent::{get_my_lines, iter_lines};
use anyhow::{Context, Result};

const MAX_AGE: usize = 9;
const RESET_AGE: usize = 6;

fn parse_input() -> Result<[usize; MAX_AGE]> {
    let mut fish: [usize; MAX_AGE] = [0; MAX_AGE];
    get_my_lines!()
        .next()
        .context("No data found!")?
        .split(',')
        .map(|i| i.parse::<usize>().map_err(anyhow::Error::new))
        .collect::<Result<Vec<usize>>>()?
        .iter()
        .for_each(|&age| fish[age] += 1);
    Ok(fish)
}

fn grow_fish(fish: &mut [usize; MAX_AGE]) {
    let ready = fish[0];
    fish[0] = 0;
    fish.rotate_left(1);
    fish[MAX_AGE - 1] += ready;
    fish[RESET_AGE] += ready;
}

fn solve(days: usize) -> Result<usize> {
    let mut fish = parse_input()?;
    for _ in 0..days {
        grow_fish(&mut fish);
    }
    Ok(fish.iter().sum())
}

fn main() -> Result<()> {
    println!("Fish after 80 days: {}", solve(80)?);
    println!("Fish after 256 days: {}", solve(256)?);
    Ok(())
}
