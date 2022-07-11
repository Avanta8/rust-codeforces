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

use std::iter::{once, repeat};

#[fastout]
pub fn solve_one() {
    input! {
        len: usize, sum: usize,
    }

    let mut largest = vec![];
    largest.extend(std::iter::repeat(9).take(sum / 9));
    if sum % 9 != 0 {
        largest.push(sum % 9);
    }
    let idx = largest.len() - 1;
    if largest.len() > len {
        println!("-1 -1");
        return;
    }
    largest.extend(std::iter::repeat(0).take(len - largest.len()));

    let mut smallest = largest.iter().copied().rev().collect::<Vec<_>>();
    if smallest[0] == 0 && sum != 0 {
        smallest[0] = 1;
        smallest[largest.len() - idx - 1] -= 1;
    }

    if smallest[0] == 0 && smallest.len() > 1 && smallest[1] == 0 {
        println!("-1 -1");
    } else {
        println!(
            "{} {}",
            smallest
                .into_iter()
                .map(|x| x.to_string())
                .collect::<String>(),
            largest
                .into_iter()
                .map(|x| x.to_string())
                .collect::<String>(),
        );
    }
}

pub fn main() {
    solve_one()
}
