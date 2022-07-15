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

fn get4(y: usize, x: usize, n: usize) -> [(usize, usize); 4] {
    [
        (y, x),
        (x, n - y - 1),
        (n - y - 1, n - x - 1),
        (n - x - 1, y),
    ]
}

#[fastout]
pub fn solve_one() {
    input! {
        n: usize,
        grid: [Bytes; n],
    }

    let grid = grid
        .into_iter()
        .map(|row| row.into_iter().map(|x| x - b'0' == 1).collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut total = 0;
    for y in 0..n {
        for x in 0..n {
            if n % 2 == 1 && y == x && x == n / 2 {
                continue;
            }
            let v = get4(y, x, n)
                .into_iter()
                .map(|(y, x)| grid[y][x])
                .filter(|&x| x)
                .count();
            total += min(v, 4 - v);
        }
    }

    println!("{}", total / 4);
}

pub fn main() {
    input! {
        t: usize
    }

    for _ in 0..t {
        solve_one();
    }
}
