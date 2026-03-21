use proconio::input;
const INF: i64 = 1 << 60;
// JOI2013D 暑い日々
// dpの遷移の最後の状態を記録する
fn main() {
    input!{d: usize, n: usize, temps: [usize; d], clothes: [(usize, usize, i64); n]}
    // max value = dp[i: days][j: clothes you wear last]
    let mut dp: Vec<Vec<i64>> = vec![vec![-INF; n]; d];
    for j in 0..n {
        if clothes[j].0 <= temps[0] && temps[0] <= clothes[j].1 {
            dp[0][j] = 0;
        }
    }
    for i in 1..d {
        // 現在の状態
        for j in 0..n {
            if clothes[j].0 <= temps[i] && temps[i] <= clothes[j].1 {
                // 前の状態 j != prev_jとする問題もある
                for prev_j in 0..n {
                    if dp[i-1][prev_j] == -INF {
                        continue;
                    }
                    let value: i64 = (clothes[prev_j].2 - clothes[j].2).abs() + dp[i-1][prev_j];
                    dp[i][j] = dp[i][j].max(value);
                }
            }
        }
    }

    let max_value: i64 = *dp[d-1].iter().max().unwrap();
    println!("{}", max_value);
}