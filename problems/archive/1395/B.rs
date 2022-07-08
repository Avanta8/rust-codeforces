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

use proconio::{fastout, input, marker::Usize1};

pub fn solve_one() -> i64 {
    unimplemented!();
}

#[fastout]
pub fn main() {
    input! {
        (h, w, sy, sx): (usize, usize, Usize1, Usize1)
    }

    let mut ans = vec![(sx, sy)];

    for px in 0..w {
        if px == sx {
            continue;
        }
        ans.push((px, sy));
    }

    let mut forw = false;
    for py in 0..h {
        if py == sy {
            continue;
        }
        let it = if forw {
            (0..w).collect::<Vec<_>>()
        } else {
            (0..w).rev().collect::<Vec<_>>()
        };

        ans.extend(it.into_iter().map(|px| (px, py)));
        forw = !forw;
    }

    for (x, y) in ans {
        println!("{} {}", y + 1, x + 1);
    }
}
