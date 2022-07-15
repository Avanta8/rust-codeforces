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

use proconio::marker::{Chars, Usize1};
use proconio::{fastout, input};

use std::cmp::*;
use std::collections::*;

#[fastout]
pub fn solve_one() {
    input! {
        len: usize, num_ops: usize, qs: usize,
        s: Chars,

        ops: [(Usize1, Usize1); num_ops],
        queries: [Usize1; qs],
    }

    let mut map = BTreeMap::new();
    let mut ln = len;
    for (start, end) in ops.iter().copied() {
        map.insert(ln, (start, end));
        ln += end - start + 1;
    }

    // println!("{:?}", map);

    for mut q in queries.iter().copied() {
        while q >= len {
            let (&start, &sec) = map.range(..=q).next_back().unwrap();
            let diff = q - start;
            q = sec.0 + diff;
        }
        println!("{}", s[q]);
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
