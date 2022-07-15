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

use std::cmp::*;
use std::collections::*;

#[fastout]
pub fn solve_one() {
    input! {
        len: usize,
        vec: [usize; len],
    }

    let mut prevs = vec![];
    let mut total = 0;
    for (i, a) in (1..).zip(vec.iter().copied()) {
        if a >= i {
            continue;
        }
        total += prevs.partition_point(|&x| a > x);
        prevs.push(i);
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
