use proconio::input;
use std::f64::MAX;
// 鉄則B23
// bitDp: 現在地とvisited(bit)を添え字にする。
fn main() {
    input!{n: usize, xy: [(f64, f64); n]}
    let calc_time = |prev: usize, next: usize| -> f64 {
        let x_square: f64 = (xy[next].0 - xy[prev].0) * (xy[next].0 - xy[prev].0);
        let y_square: f64 = (xy[next].1 - xy[prev].1) * (xy[next].1 - xy[prev].1);
        (x_square + y_square).sqrt()
    };
    let bits: usize = 1 << n;
    // min time = dp[i: cur][j: visited]
    let mut dp: Vec<Vec<f64>> = vec![vec![MAX; bits]; n];
    // 一周するので頂点0から始まるとしても問題ない。
    dp[0][0] = 0.0;

    // for文の順番をstate -> cur -> nextにする。
    for bit in 0..bits {
        for i in 0..n {
            for next in 0..n {
                // if not visited
                if next != i && bit >> next & 1 == 0 {
                    let time: f64 = calc_time(i, next);
                    let next_state: usize = bit | (1 << next);
                    dp[next][next_state] = dp[next][next_state].min(dp[i][bit] + time);
                }
            }
        }
    }

    println!("{}", dp[0][bits-1]);
}