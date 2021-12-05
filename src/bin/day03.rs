use advent::{get_my_lines, iter_lines};
use anyhow::{Result, bail};
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
        assert_eq!(self.size, line.len());
        line.chars().enumerate().for_each(|(i, ch)|
            self.freq[i][&ch] += 1
        );
        Ok(())
    }
}

fn solve() -> Result<()> {
    let mut f = FreqCounter::new(122);
    get_my_lines!().try_for_each(|line| f.update(&line))
}


fn main() {
    println!("Hello day3");
    match solve() {
        Ok(()) => println!("Success!"),
        Err(e) => println!("Something went wrong: {}", e),
    };
}