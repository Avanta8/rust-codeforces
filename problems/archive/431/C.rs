#![allow(
    unused_imports,
    non_camel_case_types,
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

use memoise::memoise;
use proconio::{fastout, input};

use std::cmp::*;
use std::collections::*;

const MOD: i32 = 1000000007;

#[memoise(n, used)]
fn calc(n: i32, k: i32, d: i32, used: bool) -> i32 {
    if n == 0 {
        return if used { 1 } else { 0 };
    }
    let mut total = 0;
    for x in 1..=k {
        if x > n {
            continue;
        }
        total = (total + calc(n - x, k, d, used || x >= d) % MOD) % MOD;
    }
    total
}

#[fastout]
pub fn solve_one() {
    input! {n: i32, k: i32, d: i32}

    println!("{}", calc(n, k, d, false));
}

pub fn main() {
    solve_one();
}
