use advent::{get_my_lines, iter_lines};
use anyhow::{anyhow, bail, Context, Result};
use counter::Counter;

fn freq_at(report: &[String], pos: usize) -> Result<Counter<u8>> {
    let f: Option<Vec<&u8>> = report.iter().map(|s| s.as_bytes().get(pos)).collect();
    let mut counter: Counter<u8> = Counter::new();
    f.with_context(|| format!("Out of bounds counting bit at position: {}", pos))?
        .iter()
        .for_each(|ch| counter[ch] += 1);
    Ok(counter)
}

fn mcb_at(report: &[String], pos: usize) -> Result<u8> {
    let counter = freq_at(report, pos)?;
    if counter[&b'1'] >= counter[&b'0'] {
        Ok(b'1')
    } else {
        Ok(b'0')
    }
}

fn lcb_at(report: &[String], pos: usize) -> Result<u8> {
    let counter = freq_at(report, pos)?;
    if counter[&b'0'] <= counter[&b'1'] {
        Ok(b'0')
    } else {
        Ok(b'1')
    }
}

fn binary_to_int(s: &str) -> Result<usize> {
    usize::from_str_radix(s, 2).context("Failed to parse binary string!")
}

fn collect_bits(vec: &[String], cond: fn(&[String], usize) -> Result<u8>) -> Result<String> {
    let len = vec.first().ok_or(anyhow!("No elements in vector!"))?.len();
    let bits: Vec<u8> = (0..len)
        .map(|i| cond(vec, i))
        .collect::<Result<Vec<u8>>>()?;
    Ok(String::from_utf8(bits)?)
}

fn filter_by_bit(vec: &[String], cond: fn(&[String], usize) -> Result<u8>) -> Result<String> {
    let mut vec = vec.to_owned();
    for bit in 0..vec.len() {
        let criterion = cond(&vec, bit)?;
        vec.retain(|item| item.as_bytes()[bit] == criterion);
        if vec.len() == 1 {
            return Ok(vec[0].to_owned());
        }
    }
    bail!("Expected one element left, found: {}!", vec.len())
}

fn solve_p1() -> Result<usize> {
    let lines: Vec<String> = get_my_lines!().collect();
    let gamma = binary_to_int(&collect_bits(&lines, mcb_at)?)?;
    let epsilon = binary_to_int(&collect_bits(&lines, lcb_at)?)?;
    println!("gamma: {}, epsilon: {}", gamma, epsilon);
    Ok(gamma * epsilon)
}

fn solve_p2() -> Result<usize> {
    let lines: Vec<String> = get_my_lines!().collect();
    let oxygen = binary_to_int(&filter_by_bit(&lines, mcb_at)?)?;
    let co2 = binary_to_int(&filter_by_bit(&lines, lcb_at)?)?;
    println!("oxygen: {}, co2: {}", oxygen, co2);
    Ok(oxygen * co2)
}

fn main() -> Result<()> {
    println!("gamma * epsilon: {}", solve_p1()?);
    println!("oxygen * co2: {}", solve_p2()?);
    Ok(())
}
