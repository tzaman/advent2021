use advent::{get_my_lines, iter_lines};
use anyhow::{bail, Context, Result};
use counter::Counter;

#[derive(Debug)]
struct FreqCounter {
    size: usize,
    freq: Vec<Counter<char>>,
}

impl FreqCounter {
    fn new(size: usize) -> Self {
        Self {
            size,
            freq: vec![Counter::new(); size],
        }
    }

    fn update(&mut self, line: &str) -> Result<()> {
        if self.size != line.len() {
            bail!("Counter size must match line length!")
        }
        line.chars()
            .enumerate()
            .for_each(|(i, ch)| self.freq[i][&ch] += 1);
        Ok(())
    }

    fn gamma(&self) -> Result<usize> {
        let common_bits = self
            .freq
            .iter()
            .map(|counter| counter.most_common_ordered()[0].0)
            .collect::<String>();
        usize::from_str_radix(&common_bits, 2).context("Failed to parse binary string!")
    }

    fn epsilon(&self) -> Result<usize> {
        let common_bits = self
            .freq
            .iter()
            .map(|counter| counter.most_common_ordered().last().unwrap().0)
            .collect::<String>();
        usize::from_str_radix(&common_bits, 2).context("Failed to parse binary string!")
    }
}

fn solve() -> Result<()> {
    let mut f = FreqCounter::new(12);
    get_my_lines!().try_for_each(|line| f.update(&line))?;
    println!("gamma * epsilon: {}", f.gamma()? * f.epsilon()?);
    Ok(())
}

fn main() {
    println!("Hello day3");
    match solve() {
        Ok(()) => println!("Success!"),
        Err(e) => println!("Something went wrong: {}", e),
    };
}
