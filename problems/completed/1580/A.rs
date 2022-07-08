#![allow(
    unused_imports,
    clippy::many_single_char_names,
    clippy::comparison_chain,
    clippy::if_same_then_else,
    clippy::if_not_else,
    clippy::ifs_same_cond,
    clippy::type_complexity,
    clippy::collapsible_if,
    clippy::collapsible_else_if
)]

use proconio::marker::Chars;
use proconio::{fastout, input};

use std::cmp::*;
use std::collections::*;

#[derive(Debug)]
struct Pre {
    pre: Vec<Vec<i32>>,
}

impl Pre {
    fn new(grid: &[Vec<i32>]) -> Self {
        let width = grid[0].len();
        let height = grid.len();
        let mut pre = vec![vec![0; width + 1]; height + 1];

        for (y, row) in grid.iter().enumerate() {
            for (x, &sq) in row.iter().enumerate() {
                pre[y + 1][x + 1] = pre[y][x + 1] + pre[y + 1][x] - pre[y][x] + sq;
            }
        }

        Self { pre }
    }

    /// Inclusive
    fn get(&self, a: (usize, usize), b: (usize, usize), ones: bool) -> i32 {
        let v = self.pre[b.1 + 1][b.0 + 1] - self.pre[a.1][b.0 + 1] - self.pre[b.1 + 1][a.0]
            + self.pre[a.1][a.0];
        if ones {
            v
        } else {
            let area = ((b.1 - a.1 + 1) * (b.0 - a.0 + 1)) as i32;
            area - v
        }
    }

    fn get_rect(&self, a: (usize, usize), b: (usize, usize)) -> i32 {
        self.get((a.0 + 1, a.1 + 1), (b.0 - 1, b.1 - 1), true)
            + self.get((a.0 + 1, a.1), (b.0 - 1, a.1), false)
            + self.get((a.0 + 1, b.1), (b.0 - 1, b.1), false)
            + self.get((a.0, a.1 + 1), (a.0, b.1 - 1), false)
            + self.get((b.0, a.1 + 1), (b.0, b.1 - 1), false)
    }
}

#[fastout]
pub fn solve_one() {
    input! {
        (height, width): (usize, usize),
        grid: [Chars; height],
    }

    let grid = grid
        .into_iter()
        .map(|row| {
            row.into_iter()
                .map(|sq| (sq == '1') as i32)
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let pre = Pre::new(&grid);

    let mut best = i32::MAX;

    for left in 0..width - 3 {
        for right in left + 3..width {
            let mut prev = i32::MAX / 2;
            let mut sp = 0;

            for i in 4..height {
                let new_sp = pre.get((left + 1, i), (right - 1, i), false);

                prev = min(
                    pre.get_rect((left, i - 4), (right, i)),
                    prev - sp
                        + (right as i32 - left as i32 - 1 - sp)
                        + new_sp
                        + (1 - grid[i - 1][left])
                        + (1 - grid[i - 1][right]),
                );
                best = min(best, prev);

                sp = new_sp
            }
        }
    }

    println!("{}", best);
}

pub fn main() {
    input! {
        t: usize
    }

    for _ in 0..t {
        solve_one();
    }
}
