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

use proconio::{fastout, input};
use rustc_hash::FxHashMap;

use std::cmp::*;
use std::collections::*;

#[fastout]
pub fn solve_one() {
    input! {
        len: usize, k: usize,
        stations: [i64; len],
        queries: [(i64, i64); k],
    }

    let mut sta = FxHashMap::default();

    for (i, s) in stations.iter().copied().enumerate() {
        let e = sta.entry(s).or_insert((i, i));
        e.0 = min(e.0, i);
        e.1 = max(e.1, i);
    }

    for (f, t) in queries {
        if sta.get(&f).map(|x| x.0).unwrap_or(usize::MAX)
            < sta.get(&t).map(|x| x.1).unwrap_or(usize::MIN)
        {
            println!("YES");
        } else {
            println!("NO");
        }
    }
}

pub fn main() {
    input! {
        t: usize
    }

    for _ in 0..t {
        solve_one();
    }
}
