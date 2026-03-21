use proconio::input;
use proconio::marker::Chars;
// 鉄則B21
// 回文: 区間DP
fn main() {
    input!{n: usize, s: Chars}
    // [l r)の最大の回文の長さ = dp[l][r]
    let mut dp: Vec<Vec<usize>> = vec![vec![0; n+1]; n+1];
    // 一文字だけ
    for i in 0..n {
        dp[i][i+1] = 1;
    }
    // 二文字だけ
    for i in 0..n {
        if i + 2 <= n {
            if s[i] == s[i+1] {
                dp[i][i+2] = 2;
            } else {
                // do not forget!!!
                dp[i][i+2] = 1;
            }
        }
    }
    // 回文の範囲を外に広げる(2から始める方が安全) <- :l  r: ->
    for span in 3..=n {
        for l in 0..n {
            let r: usize = l + span;
            if r > n {
                continue;
            }
            // left
            dp[l][r] = dp[l][r].max(dp[l+1][r]);
            // right
            dp[l][r] = dp[l][r].max(dp[l][r-1]);
            // left and right
            if s[l] == s[r-1] {
                dp[l][r] = dp[l][r].max(dp[l+1][r-1] + 2)
            }
        }
    }
    println!("{}", dp[0][n]);
}