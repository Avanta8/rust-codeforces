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
        (num_soldiers, dist, num_traps, max_time): (usize, usize, usize, usize),
        mut agilities: [i64; num_soldiers],
        mut traps: [(usize, usize, i64); num_traps],  // (location, disarm, danger)
    }

    traps.sort_unstable();

    let check = |agility| {
        let mut total = 0;
        let mut pos = 0;
        let mut traps_iter = traps
            .iter()
            .copied()
            .filter(|&(_, _, d)| d > agility)
            .peekable();

        while traps_iter.peek().is_some() {
            let location = traps_iter.peek().unwrap().0;
            total += location - 1 - pos;
            pos = location - 1;

            let mut furthest = location;
            while let Some(&(location, disarm, _danger)) = traps_iter.peek() {
                if location <= furthest {
                    furthest = max(furthest, disarm);
                    traps_iter.next();
                } else {
                    break;
                }
            }

            total += (furthest - pos) * 2;
        }
        total += dist + 1 - pos;
        total <= max_time
    };

    agilities.sort_unstable();
    agilities.reverse();

    let count = agilities.partition_point(|&a| check(a));
    println!("{}", count);
}

pub fn main() {
    solve_one();
}
