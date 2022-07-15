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

use proconio::marker::Bytes;
use proconio::{fastout, input};

use std::cmp::*;
use std::collections::*;

#[fastout]
pub fn solve_one() {
    input! {
        n: usize,
        s: Bytes,
    }
    let mut seen = vec![false; 26];
    let mut total = 0;
    for c in s {
        let c = (c - b'A') as usize;

        total += 1;
        if !seen[c] {
            total += 1;
        }

        seen[c] = true;
    }
    println!("{}", total);
}

pub fn main() {
    input! {
        t: usize
    }

    for _ in 0..t {
        solve_one();
    }
}
