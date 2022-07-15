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

// #[fastout]
// pub fn solve_one() {
//     input! {
//         len: usize, price: i64,
//         chests: [i64; len],
//     }

//     let mut good_suf = vec![0];
//     let mut bad_suf = vec![0];
//     for coins in chests.iter().copied().rev() {
//         good_suf.push(good_suf.last().unwrap() + coins);
//         bad_suf.push(bad_suf.last().unwrap() + coins / 2);
//     }
//     good_suf.reverse();
//     bad_suf.reverse();

//     println!("{:?}", good_suf);
//     println!("{:?}", bad_suf);

//     let mut split = None;
//     for (i, (&good, &bad)) in good_suf.iter().zip(bad_suf.iter()).enumerate() {
//         if good - bad <= price {
//             split = Some(i);
//             break;
//         }
//     }
//     let split = split.unwrap();

//     let mut total = good_suf[0] - good_suf[split] - price * split as i64;
//     let mut div = 1;
//     for &coins in chests[split..].iter() {
//         div *= 2;
//         total += coins / div;
//     }

//     println!("{}", total);
// }

fn get_sp2(mut n: i64) -> [i64; 64] {
    let mut i = 0;
    let mut ans = [0; 64];
    while n > 0 {
        if n & 1 == 1 {
            ans[i] = 1;
        }
        n >>= 1;
        i += 1;
    }
    ans
}

fn get_value(s: &[i64; 64]) -> i64 {
    let mut total = 0;

    let mut current = 1;
    for &v in s {
        total += current * v;
        current <<= 1;
    }

    total
}

#[fastout]
pub fn solve_one() {
    input! {
        _len: usize, price: i64,
        mut chests: [i64; _len],
    }
    chests.push(0);

    let mut pref = vec![0];
    for coins in chests.iter() {
        pref.push(pref.last().unwrap() + coins);
    }

    let mut best = 0;
    let mut sp2 = [0; 64];
    for (i, coins) in chests.iter().copied().enumerate().rev() {
        let ns = get_sp2(coins);
        for j in 0..63 {
            sp2[j] = sp2[j + 1] + ns[j + 1];
        }

        best = max(best, pref[i] - price * i as i64 + get_value(&sp2));
    }
    println!("{}", best);
}

// pub fn solve_one() {
//     input! {
//         len: usize, price: i64,
//         chests: [i64; len],
//     }
//     let mut best = 0;
//     let mut best_keys = vec![];
//     for p in 0..2i64.pow(len as u32) {
//         let keys = (0..len).map(|x| (p >> x) & 1 == 1).collect::<Vec<_>>();
//         // println!("{:?}", keys);

//         let mut total = 0;
//         let mut div = 1;
//         for (key, cost) in keys.iter().copied().zip(chests.iter().copied()) {
//             if !key {
//                 div *= 2;
//             } else {
//                 total -= price;
//             }
//             total += cost / div;
//         }
//         if total > best {
//             best_keys = keys;
//         }
//         best = max(best, total);
//     }

//     println!("{}, {:?}", best, best_keys);
// }

pub fn main() {
    input! {
        t: usize
    }

    for _ in 0..t {
        solve_one();
    }
}
