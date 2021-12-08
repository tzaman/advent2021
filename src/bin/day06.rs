use advent::{get_my_values, iter_csv_line};
use anyhow::Result;

const MAX_AGE: usize = 9;
const RESET_AGE: usize = 7;

fn parse_input() -> Result<[usize; MAX_AGE]> {
    let mut fish: [usize; MAX_AGE] = [0; MAX_AGE];
    for age in get_my_values!()? {
        fish[age.parse::<usize>()?] += 1;
    }
    Ok(fish)
}

fn grow_fish(fish: &mut [usize; MAX_AGE]) {
    fish[RESET_AGE] += fish[0];
    fish.rotate_left(1);
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
