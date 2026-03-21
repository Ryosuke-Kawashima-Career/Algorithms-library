#![allow(non_snake_case)]
use proconio::input;
// 鉄則A08
fn main() {
    input!{h: usize, w: usize, X: [[usize; w]; h], q: usize}
    // 1indexed
    let mut prefix_sums: Vec<Vec<usize>> = vec![vec![0; w+1]; h+1];
    for i in 1..=h {
        for j in 1..=w {
            // 横方向に累積を計算する。
            prefix_sums[i][j] += prefix_sums[i][j-1] + X[i-1][j-1];
        }
    }
    for i in 1..=h {
        for j in 1..=w {
            // 縦方向に累積を計算する。
            prefix_sums[i][j] += prefix_sums[i-1][j];
        }
    }

    for _ in 0..q {
        // y: a,c x: b,d
        input!{a: usize, b: usize, c: usize, d: usize}
        let sum2d: usize = {
            prefix_sums[c][d] + prefix_sums[a-1][b-1] 
            - prefix_sums[a-1][d] - prefix_sums[c][b-1]
        };

        println!("{}", sum2d);
    }
}