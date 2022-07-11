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

use proconio::marker::{Bytes, Chars};
use proconio::{fastout, input};

use std::cmp::*;
use std::collections::*;

#[fastout]
pub fn solve_one() {
    input! {
        len: usize, k: usize,
        vec: Bytes,
    }

    let vec = vec.into_iter().map(|x| x - b'0').collect::<Vec<_>>();

    let mut seq = vec.iter().copied().take(k).collect::<Vec<_>>();

    let mut correct = true;
    for (&v, &x) in vec.iter().zip(seq.iter().cycle()) {
        if x < v {
            correct = false;
            break;
        } else if x > v {
            break;
        }
    }

    if !correct {
        for i in (0..k).rev() {
            if seq[i] == 9 {
                seq[i] = 0;
            } else {
                seq[i] += 1;
                break;
            }
        }
    }

    println!("{}", len);
    println!(
        "{}",
        seq.into_iter()
            .cycle()
            .take(len)
            .map(|x| x.to_string())
            .collect::<String>()
    )
}

pub fn main() {
    solve_one();
}
