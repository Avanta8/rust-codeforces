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
use rustc_hash::FxHashMap;

use std::cmp::*;
use std::collections::*;

#[fastout]
pub fn solve_one() {
    input! {
        len: usize, k: usize, xor: i32,
        mut vec: [i32; len],
    }
    vec.sort_unstable();

    let mut seen = FxHashMap::default();
    let mut vecs = vec![vec.clone()];
    seen.insert(vec.clone(), 0);

    for i in 1..=k {
        vec = vec
            .into_iter()
            .enumerate()
            .map(|(i, x)| if i % 2 == 0 { x ^ xor } else { x })
            .collect();
        vec.sort_unstable();

        if let Some(idx) = seen.get(&vec) {
            let diff = i - idx;
            let rem = k - idx;

            let parity = rem % diff;
            vec = vecs[idx + parity].clone();
            break;
        }

        seen.insert(vec.clone(), i);
        vecs.push(vec.clone());
    }

    let sml = vec.iter().copied().min().unwrap();
    let big = vec.iter().copied().max().unwrap();
    println!("{} {}", big, sml);
}

pub fn main() {
    solve_one();
}
