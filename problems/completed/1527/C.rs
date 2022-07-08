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

// fn sum(n: i64) -> i64 {
//     n * (n + 1) / 2
// }

// // #[fastout]
// pub fn solve_one() {
//     input! {
//         n: i64,
//         vec: [i64; n],
//     }

//     let mut map = FxHashMap::default();

//     for (i, x) in vec.iter().copied().enumerate() {
//         map.entry(x).or_insert_with(Vec::new).push(i as i64);
//     }

//     let mut total = 0i64;
//     for idxs in map.values() {
//         for (i, &left) in idxs.iter().enumerate() {
//             let msr = left;
//             total -= sum(msr) * (idxs.len() - i - 1) as i64;

//             let msl = n - left - 1;
//             total -= sum(msl) * i as i64;

//             for &right in &idxs[i + 1..] {
//                 let mc = n - right + left;
//                 total += sum(mc);
//             }
//         }

//         // for (i, &right) in idxs.iter().rev().enumerate() {
//         //     let msl = n - right - 1;
//         //     total -= sum(msl) * (idxs.len() - i - 1) as i64;
//         // }
//     }
//     println!("{}", total);
// }

#[fastout]
pub fn solve_one() {
    input! {
        n: usize,
        vec: [i64; n],
    }

    let mut table = vec![0; n]; // table[i] = the sum of all (indices + 1) < i of elements that match vec[i]

    let mut totals = HashMap::new();
    for (i, &x) in vec.iter().enumerate() {
        let entry = totals.entry(x).or_insert(0);
        table[i] = *entry;
        *entry += i as i64 + 1;
    }

    let mut total = 0;
    let mut count = 0;
    for i in 1..n {
        count += table[i];
        total += count;
    }
    println!("{}", total);
}

pub fn main() {
    input! {
        t: usize
    }

    for _ in 0..t {
        solve_one();
    }
}
