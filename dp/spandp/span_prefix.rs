use proconio::input;
const INF: usize = 1 << 60;
// educational DP N: Slimes
// 区間DP + 累積和
// 累積和で区間に関する計算を高速化する
fn main() {
    input!{n: usize, a: [usize; n]}
    // min cost[l r) = dp[l][r]
    let mut dp: Vec<Vec<usize>> = vec![vec![INF; n+1]; n+1];
    for i in 0..n {
        // check initial values!!!
        dp[i][i+1] = 0;
    }
    // 累積和
    let mut prefix: Vec<usize> = vec![0; n+1];
    for i in 1..=n {
        prefix[i] = prefix[i-1] + a[i-1];
    }
    // 区間[l r)の長さ
    let span_sum = |l: usize, r: usize| -> usize {
        prefix[r] - prefix[l]
    };

    for span in 2..=n {
        for start in 0..n {
            let end: usize = start + span;
            if end > n {
                break;
            }
            // k: 中継点
            for k in start+1..end {
                dp[start][end] = dp[start][end].min(
                    dp[start][k] + dp[k][end]
                    + span_sum(start, k) + span_sum(k, end)
                );
            }
        }
    }
    println!("{}", dp[0][n]);
}