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

use memoise::memoise;
use proconio::{fastout, input};

use std::cmp::*;
use std::collections::*;

/*
For any sequence starting with a, the next values of the sequence must be a - 1, a - 2, ... 1.
Then, it's like the same task but with n - a elements, and starting with a + 1.
*/

/// Total number of permutations for a sequence of length `n`.
/*
#[memoise(n)]
fn get(n: i64) -> Result<i64, ()> {
    if n == 1 {
        return Ok(1);
    }

    let mut total: i64 = 1;
    for a in 1..n {
        total = total.checked_add(get(n - a)?).ok_or(())?;
    }

    Ok(total)
}
*/

fn get(n: i64) -> i64 {
    2i64.saturating_pow(n as u32 - 1)
}

#[fastout]
pub fn solve_one() {
    input! {
        n: i64, k: i64,
    }

    if get(n) < k {
        println!("-1");
        return;
    }

    let mut ans = vec![];
    let mut current = 1i64;
    let mut offset = 1;

    while ans.len() < n as usize {
        for a in offset..=n {
            if current.saturating_add(get(n - a)) > k {
                ans.extend((offset..=a).rev());
                offset = a + 1;
                break;
            }
            current += get(n - a);
        }
    }

    println!(
        "{}",
        ans.iter()
            .map(|x| x.to_string())
            .collect::<Vec<_>>()
            .join(" ")
    );
}

pub fn main() {
    input! {
        t: usize
    }

    for _ in 0..t {
        solve_one();
    }
}
