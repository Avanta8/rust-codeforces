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

use proconio::marker::{Chars, Usize1};
use proconio::{fastout, input};

use std::cmp::*;
use std::collections::*;

fn val(colour: char) -> i32 {
    match colour {
        'W' => 1,
        'B' => -1,
        _ => unreachable!(),
    }
}

fn get(idx: usize, values: &mut [Option<i32>], colours: &[char], graph: &[Vec<usize>]) -> i32 {
    if let Some(v) = values[idx] {
        v
    } else {
        let mut v = val(colours[idx]);
        for &c in &graph[idx] {
            v += get(c, values, colours, graph);
        }
        values[idx] = Some(v);
        v
    }
}

#[fastout]
pub fn solve_one() {
    input! {
        n: usize,
        parents: [Usize1; n - 1],
        colours: Chars
    }

    let mut graph = vec![vec![]; n];
    for (&p, i) in parents.iter().zip(1usize..) {
        graph[p].push(i);
    }

    let mut values = vec![None; n];

    get(0, &mut values, &colours, &graph);

    println!("{}", values.into_iter().filter(|x| x.unwrap() == 0).count());
}

pub fn main() {
    input! {
        t: usize
    }

    for _ in 0..t {
        solve_one();
    }
}
