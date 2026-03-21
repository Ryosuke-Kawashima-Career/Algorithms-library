use proconio::{input, marker::Chars};
const INF: usize = 1 << 60;
// abc346D
// s: 10010のように同じ文字が2連続する部分が一つしかない文字列を作る
// 0,1のスイッチの変更をxorで表現する
fn main() {
    input!{n: usize, s: Chars, c: [usize; n]}
    let s = s.iter().map(|&x| x as usize - '0' as usize).collect::<Vec<usize>>();
    // dp[item][last state: 0, 1][how many times same: 0, 1]
    let mut dp: Vec<Vec<Vec<usize>>> = vec![vec![vec![INF; 2]; 2]; n+1];
    dp[0][0][0] = 0;
    dp[0][1][0] = 0;
    for i in 1..=n {
        for j in 0..=1 {
            // 境界条件を定義する
            if i == 1 {
                if s[i-1] == j {
                    dp[i][j][0] = dp[i][j][0].min(dp[i-1][1^j][0]);
                } else {
                    dp[i][j][0] = dp[i][j][0].min(dp[i-1][1^j][0] + c[i-1]);
                }
                continue;
            }
            // すでに操作する必要がないとき
            if s[i-1] == j { // do nothing, no cost
                dp[i][j][0] = dp[i][j][0].min(dp[i-1][1^j][0]);
                dp[i][j][1] = dp[i][j][1].min(dp[i-1][1^j][1]);
                dp[i][j][1] = dp[i][j][1].min(dp[i-1][j][0]);
            } else {
                dp[i][j][0] = dp[i][j][0].min(dp[i-1][1^j][0] + c[i-1]);
                dp[i][j][1] = dp[i][j][1].min(dp[i-1][1^j][1] + c[i-1]);
                dp[i][j][1] = dp[i][j][1].min(dp[i-1][j][0] + c[i-1]);
            }
        }
    }
    let ans = dp[n][0][1].min(dp[n][1][1]);
    println!("{}", ans);
}