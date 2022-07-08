#![allow(
    unused_imports,
    clippy::many_single_char_names,
    clippy::comparison_chain,
    clippy::if_same_then_else,
    clippy::if_not_else,
    clippy::ifs_same_cond,
    clippy::type_complexity,
    clippy::collapsible_if,
    clippy::collapsible_else_if
)]

use proconio::marker::Usize1;
use proconio::{fastout, input};

use std::cmp::*;
use std::collections::*;

#[fastout]
pub fn main() {
    input! {
        n: usize, m1: usize, m2: usize,
        e1: [(Usize1, Usize1); m1],
        e2: [(Usize1, Usize1); m2],
    }

    let mut d1 = Dsu::new(n);
    let mut d2 = Dsu::new(n);

    for (a, b) in e1 {
        d1.unite(a, b);
    }
    for (a, b) in e2 {
        d2.unite(a, b);
    }

    let mut ans = vec![];

    for a in 0..n {
        for b in a + 1..n {
            if d1.is_same(a, b) || d2.is_same(a, b) {
                continue;
            }

            d1.unite(a, b);
            d2.unite(a, b);

            ans.push((a + 1, b + 1));
        }
    }

    println!("{}", ans.len());
    for (a, b) in ans {
        println!("{} {}", a, b);
    }
}

#[derive(Debug)]
pub struct Dsu {
    parent: Vec<usize>,
    size: Vec<usize>,
}

impl Dsu {
    pub fn new(size: usize) -> Self {
        Self {
            parent: (0..size).collect(),
            size: vec![1; size],
        }
    }

    pub fn is_same(&mut self, a: usize, b: usize) -> bool {
        self.get(a) == self.get(b)
    }

    pub fn get(&mut self, v: usize) -> usize {
        if v == self.parent[v] {
            return v;
        }

        self.parent[v] = self.get(self.parent[v]);
        self.parent[v]
    }

    pub fn unite(&mut self, a: usize, b: usize) -> bool {
        let mut sa = self.get(a);
        let mut sb = self.get(b);
        if sa == sb {
            return false;
        }

        if self.size[sa] < self.size[sb] {
            std::mem::swap(&mut sa, &mut sb);
        }
        self.parent[sb] = sa;
        self.size[sa] += self.size[sb];

        true
    }
}
