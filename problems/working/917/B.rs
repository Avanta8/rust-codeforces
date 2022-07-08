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

use proconio::{fastout, input, marker::Usize1};
use rustc_hash::FxHashMap;

use std::cmp::*;
use std::collections::*;

fn gen_graph(edges: &[(usize, usize, char)], len: usize) -> Vec<Vec<(usize, char)>> {
    let mut graph = vec![vec![]; len];
    for &(a, b, char) in edges {
        graph[a].push((b, char));
    }
    graph
}

struct Evaluator<'a> {
    graph: &'a [Vec<(usize, char)>],
    cache: FxHashMap<(usize, usize, char), bool>,
}

impl<'a> Evaluator<'a> {
    fn new(graph: &'a [Vec<(usize, char)>]) -> Self {
        Self {
            graph,
            cache: Default::default(),
        }
    }

    fn eval(&mut self, a: usize, b: usize, last: char) -> bool {
        self.cache.get(&(a, b, last)).copied().unwrap_or_else(|| {
            let result = self._eval(a, b, last);
            self.cache.insert((a, b, last), result);
            result
        })
    }

    /// `a`'s turn. return true if `a` can win from this state.
    fn _eval(&mut self, a: usize, b: usize, last: char) -> bool {
        if self.graph[a].is_empty() {
            return false;
        }

        for &(child, char) in &self.graph[a] {
            if char < last {
                continue;
            }
            if !self.eval(b, child, char) {
                return true;
            }
        }
        false
    }
}

#[fastout]
pub fn solve_one() {
    input! {
        size: usize, num_edges: usize,
        edges: [(Usize1, Usize1, char); num_edges],
    }

    let graph = gen_graph(&edges, size);
    let mut evaluator = Evaluator::new(&graph);

    for i in 0..size {
        for j in 0..size {
            print!("{}", if evaluator.eval(i, j, 'a') { 'A' } else { 'B' });
        }
        println!();
    }
}

pub fn main() {
    solve_one();
}
