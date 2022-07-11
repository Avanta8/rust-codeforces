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
use rustc_hash::FxHashSet;

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

fn check(graph: &[Vec<usize>], set: &FxHashSet<usize>, len: usize) -> bool {
    let first = *set.iter().next().unwrap();
    let mut prev = vec![None; len];
    prev[first] = Some(0);

    let mut bag = vec![first];

    while let Some(current) = bag.pop() {
        for &child in &graph[current] {
            if prev[child].is_some() {
                continue;
            }

            prev[child] = Some(current);
            bag.push(child);
        }
    }
    prev[first] = None;

    let mut nodes = FxHashSet::default();
    nodes.insert(first);

    for mut node in set.iter().copied() {
        nodes.insert(node);
        if node == first {
            continue;
        }

        while let Some(next) = prev[node] {
            if set.contains(&next) {
                break;
            }
            nodes.insert(next);
            node = next;
        }
    }

    for &node in nodes.iter() {
        if graph[node].iter().filter(|x| nodes.contains(x)).count() > 2 {
            return false;
        }
    }
    true
}

#[fastout]
pub fn solve_one() {
    input! {
        len: usize,
        edges: [(Usize1, Usize1); len - 1],
        q: usize,
    }

    let graph = gen_graph(&edges, len);

    for _ in 0..q {
        input! {
            k: usize,
            vec: [Usize1; k],
        }

        let set = vec.iter().copied().collect::<FxHashSet<_>>();
        println!(
            "{}",
            if check(&graph, &set, len) {
                "YES"
            } else {
                "NO"
            }
        );
    }
}

pub fn main() {
    solve_one();
}
