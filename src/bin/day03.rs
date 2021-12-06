use advent::{get_my_lines, iter_lines};
use anyhow::{bail, Context, Result};
use counter::Counter;

fn mcb_at(report: &Vec<String>, pos: usize) -> Result<u8> {
    let f: Option<Vec<&u8>> = report.iter().map(|s| s.as_bytes().get(pos)).collect();
    let mut counter: Counter<u8> = Counter::new();
    f.context("Out of bounds getting MCB")?
        .iter()
        .for_each(|ch| counter[&ch] += 1);
    if counter[&b'1'] >= counter[&b'0'] {
        Ok(b'1')
    } else {
        Ok(b'0')
    }
}

fn lcb_at(report: &Vec<String>, pos: usize) -> Result<u8> {
    let f: Option<Vec<&u8>> = report.iter().map(|s| s.as_bytes().get(pos)).collect();
    let mut counter: Counter<u8> = Counter::new();
    f.context("Out of bounds getting LCB")?
        .iter()
        .for_each(|ch| counter[ch] += 1);
    if counter[&b'0'] <= counter[&b'1'] {
        Ok(b'0')
    } else {
        Ok(b'1')
    }
}

fn binary_to_int(s: &str) -> Result<usize> {
    usize::from_str_radix(s, 2).context("Failed to parse binary string!")
}

fn solve_p1() -> Result<usize> {
    let lines: Vec<String> = get_my_lines!().collect();
    let len = lines[0].len();
    let gamma: Vec<u8> = (0..len)
        .map(|i| mcb_at(&lines, i))
        .collect::<Result<Vec<u8>>>()
        .context("mcb failed?")?;
    let gamma = String::from_utf8(gamma)?;

    let epsilon: Vec<u8> = (0..len)
        .map(|i| lcb_at(&lines, i))
        .collect::<Result<Vec<u8>>>()
        .context("lcb failed?")?;
    let epsilon = String::from_utf8(epsilon)?;

    let gamma = binary_to_int(&gamma)?;
    let epsilon = binary_to_int(&epsilon)?;
    println!("gamma: {}, epsilon: {}", gamma, epsilon);
    Ok(gamma * epsilon)
}

fn filter_by_bit(vec: &Vec<String>, cond: fn(&Vec<String>, usize) -> Result<u8>) -> Result<String> {
    let mut vec = vec.clone();
    for bit in 0..vec.len() {
        let criterion = cond(&vec, bit)?;
        vec.retain(|item| item.as_bytes()[bit] == criterion);
        if vec.len() == 1 {
            return Ok(vec.first().unwrap().to_owned());
        }
    }
    bail!("Expected one element left, found: {}!", vec.len())
}

fn solve_p2() -> Result<usize> {
    let lines: Vec<String> = get_my_lines!().collect();
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
