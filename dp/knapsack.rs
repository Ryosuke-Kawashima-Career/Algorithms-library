const INF: i64 = 1 << 60;
// DPL_1_C
// ナップザック問題: 1つの種類から複数選べる
fn main() {
    input!{n: usize, w: usize, vw: [(i64, usize); n]}
    // value = dp[i: item][j: weight]
    let mut dp: Vec<Vec<i64>> = vec![vec![-INF; w+1]; n+1];
    // 初期化する
    dp[0][0] = 0;
    for i in 1..=n {
        for j in 0..=w {
            // 何も買わないとき
            if dp[i-1][j] != -INF {
                dp[i][j] = dp[i][j].max(dp[i-1][j]);
            }
            if j >= vw[i-1].1 {
                // 一つ買うとき
                dp[i][j] = dp[i][j].max(dp[i-1][j - vw[i-1].1] + vw[i-1].0);
                // 複数買うとき
                dp[i][j] = dp[i][j].max(dp[i][j - vw[i-1].1] + vw[i-1].0);
            }
        }
    }
    let ans: i64 = *dp[n].iter().max().unwrap();
    println!("{}", ans);
}