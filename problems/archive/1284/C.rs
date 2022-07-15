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
        n: i64, m: i64,
    }

    let mut permutations = vec![1];
    for x in 1..=n {
        permutations.push((permutations.last().unwrap() * x) % m);
    }

    let mut total = 0;
    for len in 1..=n {
        let perms = (permutations[(n - len) as usize] * permutations[len as usize]) % m;
        let counts = n - len + 1;
        total = (total + ((counts * counts) % m) * perms) % m;
    }
    println!("{}", total);
}

pub fn main() {
    solve_one();
}
