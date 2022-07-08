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

use proconio::{fastout, input};

use std::cmp::*;
use std::collections::*;

#[fastout]
pub fn main() {
    input! {
        mut points: [(i32, i32); 3]
    }

    points.sort_unstable_by_key(|&(x, _y)| x);

    let a = points[0];
    let b = points[1];
    let c = points[2];

    let mut trees = HashSet::new();

    trees.extend((a.0..=b.0).map(|x| (x, a.1)));
    trees.extend((min(a.1, b.1)..=max(a.1, b.1)).map(|y| (b.0, y)));
    trees.extend((min(b.1, c.1)..=max(b.1, c.1)).map(|y| (b.0, y)));
    trees.extend((b.0..=c.0).map(|x| (x, c.1)));

    println!("{}", trees.len());
    for (x, y) in trees {
        println!("{} {}", x, y);
    }
}
