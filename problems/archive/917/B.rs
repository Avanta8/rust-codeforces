#![allow(
    unused_imports,
    clippy::many_single_char_names,
    clippy::comparison_chain,
    clippy::if_same_then_else,
    clippy::if_not_else,
    clippy::ifs_same_cond,
    clippy::type_complexity,
    clippy::collapsible_if,
    clippy::collapsible_else_if,
    clippy::needless_range_loop
)]

use memoise::memoise;
use proconio::{fastout, input, marker::Usize1};

use std::cmp::*;
use std::collections::*;

fn gen_graph(edges: &[(usize, usize, char)], len: usize) -> Vec<Vec<(usize, char)>> {
    let mut graph = vec![vec![]; len];
    for &(a, b, char) in edges {
        graph[a].push((b, char));
    }
    graph
}

#[memoise(a, b, last)]
fn eval(graph: &[Vec<(usize, char)>], a: usize, b: usize, last: char) -> bool {
    if graph[a].is_empty() {
        return false;
    }

    for &(child, char) in &graph[a] {
        if char < last {
            continue;
        }
        if !eval(graph, b, child, char) {
            return true;
        }
    }
    false
}

#[fastout]
pub fn solve_one() {
    input! {
        size: usize, num_edges: usize,
        edges: [(Usize1, Usize1, char); num_edges],
    }

    let graph = gen_graph(&edges, size);

    for i in 0..size {
        for j in 0..size {
            print!("{}", if eval(&graph, i, j, 'a') { 'A' } else { 'B' });
        }
        println!();
    }
}

pub fn main() {
    solve_one();
}
