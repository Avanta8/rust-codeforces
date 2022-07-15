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
        mut vec: [i64; len],
    }

    vec.pop();
    vec = vec.into_iter().skip_while(|&x| x == 0).collect();

    let zeros = vec.iter().filter(|&&x| x == 0).count() as i64;
    let sum = vec.iter().sum::<i64>();

    // println!("{:?}", vec);
    println!("{}", zeros + sum);
}

pub fn main() {
    input! {
        t: usize
    }

    for _ in 0..t {
        solve_one();
    }
}
