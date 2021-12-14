use advent::{get_my_lines, iter_lines};
use anyhow::{Context, Result};
use itertools::Itertools;
use std::collections::{HashMap, HashSet};
use std::ops::Deref;

#[derive(Debug, Default)]
struct Graph(HashMap<String, HashSet<String>>);

impl Deref for Graph {
    type Target = HashMap<String, HashSet<String>>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Graph {
    fn add_edge(&mut self, src: &str, dst: &str) {
        self.0
            .entry(src.to_string())
            .or_default()
            .insert(dst.to_string());
    }

    fn add_undirected_edge(&mut self, left: &str, right: &str) {
        self.add_edge(left, right);
        self.add_edge(right, left);
    }

    fn count_paths(&self, have_time: bool) -> usize {
        self.traverse("start", &mut Vec::new(), have_time)
    }

    fn traverse(&self, node: &str, path: &mut Vec<String>, have_time: bool) -> usize {
        if node == "end" {
            return 1;
        }
        path.push(node.to_string());
        let mut total = 0;
        for entry in self[node].iter() {
            let self_visits = path.iter().filter(|&e| e == entry).count();
            if entry.chars().all(|c| c.is_uppercase()) || self_visits == 0 {
                total += self.traverse(entry, path, have_time);
            } else if have_time && entry != "start" && self_visits == 1 {
                total += self.traverse(entry, path, false);
            }
        }
        path.pop();
        total
    }
}

fn parse_input() -> Result<Graph> {
    let mut graph: Graph = Graph::default();
    for line in get_my_lines!() {
        let (a, b) = line.split('-').next_tuple().context("Failed to split!")?;
        graph.add_undirected_edge(a, b)
    }
    Ok(graph)
}

fn main() -> Result<()> {
    let g = parse_input()?;
    println!("Paths (p1): {}", g.count_paths(false));
    println!("Paths (p2): {}", g.count_paths(true));
    Ok(())
}
