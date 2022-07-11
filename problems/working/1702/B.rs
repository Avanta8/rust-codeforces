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
        s: Chars,
    }
    let mut days = 1;
    let mut bag = HashSet::new();

    for c in s {
        if bag.len() == 3 {
            if !bag.contains(&c) {
                bag.clear();
                bag.insert(c);
                days += 1;
            }
        } else {
            bag.insert(c);
        }
    }

    println!("{}", days);
}

pub fn main() {
    input! {
        t: usize
    }

    for _ in 0..t {
        solve_one();
    }
}
