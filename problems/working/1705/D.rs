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

use std::cmp::*;
use std::collections::*;

/*
It is possible iff the order of groups are the same
*/

fn group(vec: Vec<bool>) -> Vec<(bool, (usize, usize))> {
    let mut groups = vec![];
    let mut prev = vec[0];
    let mut pos = (0, 0);
    for (i, item) in vec.into_iter().enumerate().skip(1) {
        if item == prev {
            pos.1 += 1;
        } else {
            groups.push((prev, pos));
            prev = item;
            pos = (i, i);
        }
    }
    groups.push((prev, pos));
    groups
}

#[fastout]
pub fn solve_one() {
    input! {
        _len: usize,
        vec: Bytes,
        target: Bytes,
    }

    let vec = vec.into_iter().map(|x| (x - b'0') == 1).collect::<Vec<_>>();
    let target = target
        .into_iter()
        .map(|x| (x - b'0') == 1)
        .collect::<Vec<_>>();

    // println!("{:?}", vec);
    // println!("{:?}", target);

    let vec_groups = group(vec);
    let target_groups = group(target);

    // println!("{:?}", vec_groups);
    // println!("{:?}", target_groups);

    if vec_groups.len() != target_groups.len() {
        println!("-1");
        return;
    }

    let mut total = 0;
    for ((a, apos), (b, bpos)) in vec_groups.into_iter().zip(target_groups) {
        if a != b {
            println!("-1");
            return;
        }
        total += (apos.1 as i64 - bpos.1 as i64).abs();
    }

    println!("{}", total);
}

pub fn main() {
    input! {
        t: usize
    }

    for _ in 0..t {
        solve_one();
    }
}
