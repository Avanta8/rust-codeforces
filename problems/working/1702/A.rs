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
        m: i64,
    }
    let s = m.to_string();
    let l = s.len() - 1;
    let p = (10i64).pow(l as u32);

    println!("{}", m - p);
}

pub fn main() {
    input! {
        t: usize
    }

    for _ in 0..t {
        solve_one();
    }
}
