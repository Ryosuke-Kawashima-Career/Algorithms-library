#![allow(non_snake_case)]
use proconio::input;
use proconio::marker::Chars;
use proconio::fastout;
// interactive
use std::io::{stdin, stdout, BufRead, BufReader, Write};
use proconio::{input, source::line::LineSource};
// interactive3
use proconio::input_interactive;

#[fastout]
fn main() {
    input!{l: usize, r: usize}
    let (l, r) = (l-1, r-1);
    // input: [a a b b] with white space
    input!{c: [char; n]}

    // interactive1
    let stdin = stdin();
    let mut source = LineSource::new(BufReader::new(stdin.lock()));
    input! {
        from &mut source,
        n: usize,
    }
    stdout().flush().unwrap();

    // interactive2
    let reply = query(x, &mut source);

    // interactive3
    input_interactive!{n: usize}
}

// interactive2
fn query<R: BufRead>(x: usize, source: &mut LineSource<R>) -> usize {
    println!("{}", x);
    stdout().flush().unwrap();
    input! {
        from source,
        y: usize,
    }
    y
}

use proconio::derive_readable;

#[derive_readable]
#[derive(Debug, Copy, Clone)]
struct Point {
    x: usize, y: usize
}

impl Point {
    fn new(x: usize, y: usize) -> Self {
        Self{x: x, y: y}
    }
}

let calc_dist = |i: usize, j: usize| -> f64 {
    let dist_square_x: f64 = (points[i].x - points[j].x) * (points[i].x - points[j].x);
    let dist_square_y: f64 = (points[i].y - points[j].y) * (points[i].y - points[j].y);
    let dist_xy = (dist_square_x + dist_square_y).sqrt();
    return dist_xy;
};

#[derive(Debug, Clone)]
struct Input {
    A: usize, B: usize, a: Vec<usize>, b: Vec<usize>
}

// 入力や前計算を管理する構造体
/* let input: Input = Input::new(); */
#[derive(Debug, Clone)]
struct Input {
    n: usize,
    a: Vec<usize>,
}

impl Input {
    fn new() -> Input {
        input! {
            n: usize,
            a: [usize; n],
        }
        Input {
            n,
            a,
        }
    }
}

for &Edge{to, cost} in graph[vertex].iter() {
}