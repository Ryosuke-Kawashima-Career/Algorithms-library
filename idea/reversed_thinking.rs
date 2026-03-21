use proconio::{input, marker::Chars};
use std::collections::HashSet;
// abc398d
// A. 点ではなく座標(地面)を動かす
fn main() {
    input!{n: i64, r: i64, c: i64, s: Chars}
    // the fire is at (0, 0) at first
    let mut coord: HashSet<(i64, i64)> = HashSet::new();
    coord.insert((0, 0));
    let mut answer: Vec<usize> = Vec::new();
    let mut dy: i64 = 0;
    let mut dx: i64 = 0;
    for t in 0..(n as usize) {
        // move in the opposite
        if s[t] == 'N' {
            dy += 1;
        } else if s[t] == 'W' {
            dx += 1;
        } else if s[t] == 'S' {
            dy -= 1;
        } else {
            dx -= 1;
        }
        coord.insert((dy, dx));
        if coord.contains(&(r + dy, c + dx)) {
            answer.push(1);
        } else {
            answer.push(0);
        }
    }

    // output
    for &ans in answer.iter() {
        print!("{}", ans);
    }
    println!("");
}