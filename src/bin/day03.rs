use advent::{get_my_lines, iter_lines};
use anyhow::{bail, Context, Result};
use counter::Counter;

#[derive(Debug)]
struct FreqCounter {
    size: usize,
    freq: Vec<Counter<char>>,
}

fn mcb_at(report: &Vec<String>, pos: usize) -> Result<u8> {
    let f: Vec<u8> = report.iter().map(|s| s.as_bytes()[pos]).collect();
    let mut counter: Counter<u8> = Counter::new();
    f.iter().for_each(|ch| counter[&ch] += 1);
    if counter[&b'1'] >= counter[&b'0'] {
        Ok(b'1')
    } else {
        Ok(b'0')
    }
}

fn lcb_at(report: &Vec<String>, pos: usize) -> Result<u8> {
    let f: Vec<u8> = report.iter().map(|s| s.as_bytes()[pos]).collect();
    let mut counter: Counter<u8> = Counter::new();
    f.iter().for_each(|ch| counter[&ch] += 1);
    if counter[&b'0'] <= counter[&b'1'] {
        Ok(b'0')
    } else {
        Ok(b'1')
    }
}

fn mcb(fc: &Counter<char>) -> Result<char> {
    Ok(fc
        .most_common()
        .first()
        .context("Failed to fetch most common element!")?
        .0)
}

fn lcb(fc: &Counter<char>) -> Result<char> {
    Ok(fc
        .most_common()
        .last()
        .context("Failed to fetch least common element!")?
        .0)
}

fn binary_to_int(s: &str) -> Result<usize> {
    usize::from_str_radix(s, 2).context("Failed to parse binary string!")
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

    fn gamma(&self) -> Result<String> {
        self.freq.iter().map(mcb).collect::<Result<String>>()
    }

    fn epsilon(&self) -> Result<String> {
        self.freq.iter().map(lcb).collect::<Result<String>>()
    }
}

fn solve_p1() -> Result<usize> {
    let mut f = FreqCounter::new(12);
    get_my_lines!().try_for_each(|line| f.update(&line))?;

    let gamma = binary_to_int(&f.gamma()?)?;
    let epsilon = binary_to_int(&f.epsilon()?)?;
    println!("gamma: {}, epsilon: {}", gamma, epsilon);
    Ok(gamma * epsilon)
}

fn filter_by_bit(vec: &Vec<String>, cond: fn(&Vec<String>, usize) -> Result<u8>) -> Result<String> {
    let mut vec = vec.clone();
    for bit in 0..vec.len() {
        let criterion = cond(&vec, bit)?;
        vec = vec
            .into_iter()
            .filter(|line| line.as_bytes()[bit] == criterion)
            .collect();
        if vec.len() == 1 {
            return Ok(vec.first().unwrap().to_owned());
        }
    }
    bail!("Expected one element left, found: {}!", vec.len())
}

fn solve_p2() -> Result<usize> {
    let lines: Vec<String> = get_my_lines!().collect();
    let mut f = FreqCounter::new(12);
    get_my_lines!().try_for_each(|line| f.update(&line))?;

    let oxygen = binary_to_int(&filter_by_bit(&lines, mcb_at)?)?;
    let co2 = binary_to_int(&filter_by_bit(&lines, lcb_at)?)?;
    println!("Oxygen: {}, CO2: {}", oxygen, co2);
    Ok(oxygen * co2)
}

fn main() {
    match solve_p1() {
        Ok(result) => println!("gamma * epsilon: {}", result),
        Err(e) => println!("Something went wrong: {}", e),
    };
    match solve_p2() {
        Ok(result) => println!("oxygen * co2: {}", result),
        Err(e) => println!("Something went wrong: {}", e),
    };
}
