use proconio::input;
use std::collections::BinaryHeap;
// 鉄則B39
// 時が来たら封印されし力を開放する
fn main() {
    input!{n: usize, d: usize, xy: [(usize, usize); n]}
    // release[day of publication][reward]
    let mut release: Vec<Vec<usize>> = vec![vec![]; d+1];
    for &(x, y) in xy.iter() {
        release[x].push(y);
    }
    let mut ans: usize = 0;
    let mut bh = BinaryHeap::new();
    for day in 1..=d {
        // if the work is published
        for &reward in release[day].iter() {
            bh.push(reward);
        }
        if let Some(max_reward) = bh.pop() {
            ans += max_reward;
        }
    }
    println!("{}", ans);
}