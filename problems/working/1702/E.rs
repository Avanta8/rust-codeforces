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

use proconio::marker::Usize1;
use proconio::{fastout, input};
use rustc_hash::{FxHashMap, FxHashSet};

use std::cmp::*;
use std::collections::*;

pub fn gen_graph(edges: &[(usize, usize)], len: usize) -> Vec<Vec<usize>> {
    let mut graph = vec![vec![]; len];
    for &(a, b) in edges {
        graph[a].push(b);
        graph[b].push(a);
    }
    graph
}

fn search(graph: &[Vec<usize>], seen: &mut [bool], node: usize) -> bool {
    let mut bag = vec![node];
    // let mut seen = FxHashSet::default();
    let mut cycle = false;
    let mut uniq = 0;
    seen[node] = true;
    while let Some(current) = bag.pop() {
        // println!("{}", current);
        uniq += 1;
        for &child in &graph[current] {
            if seen[child] {
                cycle = true;
                continue;
            }

            seen[child] = true;
            bag.push(child);
        }
    }

    // dbg!((node, cycle, uniq, seen));

    !cycle || uniq % 2 == 0
}

#[fastout]
pub fn solve_one() {
    input! {
        len: usize,
        dominoes: [(Usize1, Usize1); len],
    }

    let mut counts = FxHashMap::default();
    for (a, b) in dominoes.iter().copied() {
        if a == b {
            println!("NO");
            return;
        }
        for x in [a, b] {
            let c = counts.entry(x).or_insert(0);
            *c += 1;
            if *c > 2 {
                println!("NO");
                return;
            }
        }
    }
    for &v in counts.values() {
        if v > 2 {
            println!("NO");
            return;
        }
    }
    // println!("{:?}", counts);

    let mut seen = vec![false; len];

    let graph = gen_graph(&dominoes, len);

    for i in 0..len {
        if counts.contains_key(&i) && !seen[i] {
            if !search(&graph, &mut seen, i) {
                // println!("{:?}", i);
                println!("NO");
                return;
            }
        }
    }

    println!("YES");
}

pub fn main() {
    input! {
        t: usize
    }

    for _ in 0..t {
        solve_one();
    }
}
