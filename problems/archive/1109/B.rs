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

fn is_palindrome(s: &[u8]) -> bool {
    for i in 0..s.len() / 2 {
        if s[i] != s[s.len() - i - 1] {
            return false;
        }
    }
    true
}

#[fastout]
pub fn solve_one() {
    input! {
        string: Bytes
    }

    for i in 1..string.len() {
        let slice = [&string[i..], &string[..i]].concat();

        if slice != string && is_palindrome(&slice) {
            println!("1");
            return;
        }
    }

    for i in 1..(string.len() + 1) / 2 {
        if !is_palindrome(&string[..i]) {
            println!("2");
            return;
        }
    }

    println!("Impossible");
}

pub fn main() {
    solve_one();
}
