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
        n: usize
    }

    let mut w = 0;
    let mut h = 0;

    for _ in 0..n {
        input! {
            q: char, mut x: usize, mut y: usize
        }

        if x < y {
            std::mem::swap(&mut x, &mut y);
        }

        match q {
            '+' => {
                w = max(w, x);
                h = max(h, y);
            }
            '?' => {
                if x >= w && y >= h {
                    println!("YES");
                } else {
                    println!("NO");
                }
            }
            _ => unreachable!(),
        }
    }
}
