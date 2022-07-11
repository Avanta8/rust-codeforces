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

use proconio::marker::Bytes;
use proconio::{fastout, input};
use rustc_hash::FxHashSet;

use std::cmp::*;
use std::collections::*;

#[fastout]
pub fn solve_one() {
    input! {
        w: Bytes,
        p: i64,
    }

    let w = w
        .into_iter()
        .map(|c| (c - b'a') as i64 + 1)
        .collect::<Vec<_>>();

    let mut letters = w.iter().copied().enumerate().collect::<Vec<_>>();
    letters.sort_unstable_by_key(|x| x.1);
    letters.reverse();

    let mut it = letters.iter().copied().peekable();

    // println!("{:?}", w);
    // println!("{:?}", letters);

    let mut to_remove = FxHashSet::default();

    let mut price = w.iter().copied().sum::<i64>();
    while price > p {
        let (idx, c) = it.next().unwrap();
        to_remove.insert(idx);
        price -= c;
    }

    let mut ans = vec![];
    for (i, c) in w.iter().copied().enumerate() {
        if !to_remove.contains(&i) {
            ans.push(c);
        }
    }

    println!(
        "{}",
        ans.into_iter()
            .map(|x| ((x as u8 - 1) + b'a') as char)
            .collect::<String>()
    )
}

pub fn main() {
    input! {
        t: usize
    }

    for _ in 0..t {
        solve_one();
    }
}
