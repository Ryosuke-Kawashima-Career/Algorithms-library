use proconio::input;
const INF: i64 = 1 << 60;
// 典型A35
// min-max法: DP(先手と後手で戦略が違う)
// 先手はスコアを最大化し、後手はスコアを最小化する
fn main() {
    input!{n: usize, a: [i64; n]}
    // score = dp[i: phase]
    let mut dp: Vec<Vec<i64>> = vec![vec![0; n]; n];
    for i in 0..n {
        dp[n-1][i] = a[i];
    }

    // 後ろから考える。
    for phase in (0..n-1).rev() {
        for i in 0..=phase {
            // 先手のターンの時
            if phase % 2 == 0 {
                dp[phase][i] = dp[phase+1][i].max(dp[phase+1][i+1]);
            } else {
                dp[phase][i] = dp[phase+1][i].min(dp[phase+1][i+1]);
            }
        }
    }

    println!("{}", dp[0][0]);
}