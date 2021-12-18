use advent::{get_my_lines, iter_lines};
use anyhow::{Context, Result};
use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;
use std::cmp::max;
use std::collections::HashSet;

#[derive(Clone, Copy, Debug)]
enum Axis {
    X,
    Y,
}
type Point = (usize, usize);
type Instruction = (Axis, usize);

fn parse_input() -> Result<(HashSet<Point>, Vec<Instruction>)> {
    let mut dots = HashSet::new();
    let mut instructions = Vec::new();
    for line in get_my_lines!() {
        if let Some((a, b)) = line.split(',').next_tuple() {
            dots.insert((a.parse()?, b.parse()?));
        } else if !line.is_empty() {
            instructions.push(parse_instruction(&line)?);
        }
    }
    Ok((dots, instructions))
}

fn parse_instruction(s: &str) -> Result<Instruction> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"fold along ([x|y])=(\d+)").unwrap();
    }
    let capture = RE.captures(s).context("Failed to parse instruction")?;
    let axis = match capture.get(1).unwrap().as_str() {
        "x" => Axis::X,
        "y" => Axis::Y,
        _ => unreachable!(),
    };
    let at: usize = capture.get(2).unwrap().as_str().parse().unwrap();
    Ok((axis, at))
}

fn fold_point((x, y): Point, (axis, at): Instruction) -> Point {
    match axis {
        Axis::X => (if x < at { x } else { at * 2 - x }, y),
        Axis::Y => (x, if y < at { y } else { at * 2 - y }),
    }
}

fn render(points: &HashSet<Point>) -> Result<()> {
    let (max_x, max_y) = points
        .iter()
        .copied()
        .reduce(|acc, p| (max(acc.0, p.0), max(acc.1, p.1)))
        .context("No points!")?;
    for y in 0..=max_y {
        for x in 0..=max_x {
            print!("{}", if points.contains(&(x, y)) { "#" } else { " " });
        }
        println!();
    }
    Ok(())
}

fn solve_p1(points: &HashSet<Point>, instructions: &[Instruction]) -> Result<usize> {
    let inst = instructions.get(0).context("No instructions found!")?;
    let points: HashSet<Point> = points.iter().map(|p| fold_point(*p, *inst)).collect();
    Ok(points.len())
}

fn solve_p2(points: &HashSet<Point>, instructions: &[Instruction]) -> Result<()> {
    let mut points = points.clone();
    for inst in instructions.iter() {
        points = points.iter().map(|p| fold_point(*p, *inst)).collect();
    }
    render(&points)
}

fn main() -> Result<()> {
    let (points, instructions) = parse_input()?;
    println!("Points after 1 fold: {}", solve_p1(&points, &instructions)?);
    solve_p2(&points, &instructions)?;
    Ok(())
}
