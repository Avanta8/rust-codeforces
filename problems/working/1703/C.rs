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

use proconio::marker::Chars;
use proconio::{fastout, input};

use std::cmp::*;
use std::collections::*;

#[fastout]
pub fn solve_one() {
    input! {
        n: usize,
        vec: [i8; n],
        all_moves: [(usize, Chars); n],
    }

    for (mut val, (_, moves)) in vec.into_iter().zip(all_moves) {
        for mov in moves {
            let d = if mov == 'U' { -1 } else { 1 };
            val = (val + d).rem_euclid(10);
        }
        print!("{} ", val);
    }
    println!();
}

pub fn main() {
    input! {
        t: usize
    }

    for _ in 0..t {
        solve_one();
    }
}
