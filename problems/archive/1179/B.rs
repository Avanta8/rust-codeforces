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
        (height, width): (usize, usize),
    }

    let order = {
        let mut nums = (0..width).collect::<VecDeque<_>>();
        let mut order = vec![];
        while !nums.is_empty() {
            if let Some(n) = nums.pop_front() {
                order.push(n);
            }
            if let Some(n) = nums.pop_back() {
                order.push(n);
            }
        }
        order
    };

    let mut ans = vec![];

    for y in 0..height / 2 {
        let mut it = [y, height - y - 1].into_iter().cycle();

        ans.extend(
            order
                .iter()
                .chain(order.iter().rev())
                .map(|&x| (it.next().unwrap(), x)),
        )
    }

    if height % 2 == 1 {
        ans.extend(order.iter().map(|&x| (height / 2, x)));
    }

    for (r, c) in ans {
        println!("{} {}", r + 1, c + 1);
    }
}

pub fn main() {
    solve_one();
}
