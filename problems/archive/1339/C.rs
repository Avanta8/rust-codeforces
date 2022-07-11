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

use std::cmp::*;
use std::collections::*;

#[fastout]
pub fn solve_one() {
    input! {
        len: usize,
        vec: [i64; len],
    }

    let mut highest = i64::MIN;
    let mut diff = 0;

    for x in vec {
        highest = max(highest, x);
        diff = max(diff, highest - x);
    }

    let ans = 64 - diff.leading_zeros();
    println!("{:?}", ans);
}

pub fn main() {
    input! {
        t: usize
    }

    for _ in 0..t {
        solve_one();
    }
}
