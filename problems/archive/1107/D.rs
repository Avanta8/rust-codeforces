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

use fixedbitset::FixedBitSet;
use proconio::marker::Chars;
use proconio::{fastout, input};

use std::cmp::*;
use std::collections::*;

fn get_primes(n: usize) -> Vec<usize> {
    let mut sieve = vec![true; n + 1];
    sieve[0] = false;
    sieve[1] = false;

    for x in 0..n {
        if sieve[x] {
            for m in (x + x..n + 1).step_by(x) {
                sieve[m] = false;
            }
        }
    }
    sieve
        .into_iter()
        .zip(0..)
        .filter_map(|(x, i)| x.then(|| i))
        .collect()
}

fn reduce(grid: &[FixedBitSet], n: usize, k: usize) -> Option<Vec<FixedBitSet>> {
    let mut new_grid = vec![FixedBitSet::with_capacity(n / k); n / k];
    for (y, rows) in grid.chunks_exact(k).enumerate() {
        let xc = (0..n).collect::<Vec<_>>();
        for (x, col_idxs) in xc.chunks(k).enumerate() {
            let target = rows[0][col_idxs[0]];
            for &col in col_idxs {
                for row in rows {
                    if row[col] != target {
                        return None;
                    }
                }
            }
            new_grid[y].set(x, target);
        }
    }
    Some(new_grid)
}

#[fastout]
pub fn solve_one() {
    input! {
        mut n: usize,
        com: [Chars; n],
    }

    let mut grid = com
        .into_iter()
        .map(|row| {
            let v = row
                .into_iter()
                .map(|c| c.to_digit(16).unwrap())
                .collect::<Vec<_>>();
            let blocks = v
                .chunks(8)
                .map(|chunk| {
                    let mut count = 0;
                    let mut val = 0u32;
                    for x in chunk {
                        val <<= 4;
                        val += x;
                        count += 1;
                    }
                    val <<= 4 * (8 - count);
                    val.reverse_bits()
                })
                .collect::<Vec<_>>();
            FixedBitSet::with_capacity_and_blocks(n, blocks)
        })
        .collect::<Vec<_>>();

    let mut total = 1;
    for p in get_primes(n) {
        if n % p != 0 {
            continue;
        }
        while let Some(new_grid) = reduce(&grid, n, p) {
            total *= p;
            grid = new_grid;
            n /= p;
            if n % p != 0 {
                break;
            }
        }
    }

    println!("{}", total);
}

pub fn main() {
    solve_one();
}
