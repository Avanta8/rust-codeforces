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
use rustc_hash::FxHashSet;

use std::cmp::*;
use std::collections::*;

#[fastout]
pub fn solve_one() {
    input! {
        n: usize,
        vec: [Chars; n],
    }

    let all_strings = vec.iter().cloned().collect::<FxHashSet<_>>();

    'outer: for string in vec.iter() {
        for i in 1..string.len() {
            let start = &string[..i];
            let end = &string[i..];
            if all_strings.contains(start) && all_strings.contains(end) {
                print!("1");
                continue 'outer;
            }
        }
        print!("0");
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
