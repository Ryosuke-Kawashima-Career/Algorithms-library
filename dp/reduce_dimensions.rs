use proconio::input;
const INF: i64 = 1 << 60;
const LIMIT: usize = 250000;
// abc431D
// Q. You have to choose either of the two choices (H or W ?)
// A. Reduce the number of variables!!!
// = 制約条件でdpの次元を一つ削減することができる。
fn main() {
    input!{n: usize, whb: [(usize, i64, i64); n]}
    let mut dp: Vec<Vec<i64>> = vec![vec![-INF; LIMIT+1]; n + 1];
    dp[0][0] = 0;
    let mut sum_weight: usize = 0;
    for i in 1..=n {
        let (w, h, b) = whb[i - 1];
        for j in 0..=LIMIT {
            // `dp[i][j] = dp[i - 1][j];`　is unnecessary!
            if j >= w && dp[i - 1][j - w] != -INF {
                dp[i][j] = dp[i][j].max(dp[i - 1][j - w] + h);
            }
            dp[i][j] = dp[i][j].max(dp[i - 1][j] + b);
        }
        sum_weight += w;
    }
    let mut ans: i64 = 0;
    for j in 0..=LIMIT {
        // if the head is small enough
        if sum_weight >= 2 * j {
            ans = ans.max(dp[n][j]);
        }
    }
    println!("{}", ans);
}