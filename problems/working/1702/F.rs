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

/// (mul, exp)  mul * 2 ^ exp = n
fn get_mp2(mut n: i64) -> (i64, i64) {
    let mut exp = 0;
    while n % 2 == 0 {
        n /= 2;
        exp += 1;
    }
    (n, exp)
}

#[fastout]
pub fn solve_one() {
    input! {
        len: usize,
        target: [i64; len],
        vec: [i64; len],
    }

    let mut need = FxHashMap::default();
    for &t in target.iter() {
        let k = get_mp2(t).0;
        *need.entry(k).or_insert(0i64) += 1;
    }

    'outer: for mut x in vec {
        while x > 0 {
            if let Some(v) = need.get_mut(&x) {
                *v -= 1;
                if *v == 0 {
                    need.remove(&x);
                }
                continue 'outer;
            } else {
                x /= 2;
            }
        }
        println!("NO");
        return;
    }

    println!("YES");
}

pub fn main() {
    input! {
        t: usize
    }

    for _ in 0..t {
        solve_one();
    }
}
