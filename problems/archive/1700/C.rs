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

    let mut prev = i64::MAX;
    let mut current = 0;
    for x in vec.iter_mut() {
        *x -= current;

        if *x <= prev {
            prev = *x;
            continue;
        }

        let diff = *x - prev;
        current += diff;

        *x = prev;
    }

    let sml = vec.iter().min().unwrap().to_owned();
    let big = vec.iter().max().unwrap().to_owned();

    current += big - sml;
    current += sml.abs();

    println!("{}", current);
}

pub fn main() {
    input! {
        t: usize
    }

    for _ in 0..t {
        solve_one();
    }
}
