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
        m: i64, n: i32,
    }

    let mut total = 0f64;
    for x in 1..=m {
        let x = x as f64;
        total += x * ((x / m as f64).powi(n) - ((x - 1.) / m as f64).powi(n));
    }
    println!("{}", total);
}

pub fn main() {
    solve_one();
}
