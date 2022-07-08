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
use rustc_hash::FxHashSet;

use std::cmp::*;
use std::collections::*;

#[fastout]
pub fn solve_one() {
    input! {
        len: usize,
        sa: Bytes,
        sb: Bytes,
    }

    let mut moves = BTreeMap::new();
    for (&a, &b) in sa.iter().zip(sb.iter()) {
        if a > b {
            println!("-1");
            return;
        }
        if a < b {
            moves
                .entry((a - b'a') as usize)
                .or_insert_with(Vec::new)
                .push((b - b'a') as usize);
        }
    }

    let mut parent = (0..20).collect::<Vec<_>>();
    fn get(parent: &mut [usize], c: usize) -> usize {
        if c == parent[c] {
            return c;
        }
        parent[c] = get(parent, parent[c]);
        parent[c]
    }

    let mut ans = 0;
    for (char, targets) in moves.into_iter().rev() {
        let mut changes = FxHashSet::default();
        for t in targets {
            changes.insert(get(&mut parent, t));
        }
        ans += changes.len();
        for c in changes {
            parent[c] = char;
        }
    }

    println!("{}", ans);
}

pub fn main() {
    input! {
        t: usize
    }

    for _ in 0..t {
        solve_one();
    }
}
