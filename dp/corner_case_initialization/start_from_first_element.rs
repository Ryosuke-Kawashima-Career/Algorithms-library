use proconio::{input, marker::Chars};
const INF: usize = 1 << 60;
// abc346D
// dp[item: 0]の0が空集合か?初めの要素か?
// s: 10010のように同じ文字が2連続する部分が一つしかない文字列を作る
// 0,1のスイッチの変更をxorで表現する
fn main() {
    input!{n: usize, s: Chars, c: [usize; n]}
    let s = s.iter().map(|&x| x as usize - '0' as usize).collect::<Vec<usize>>();
    // dp[item: 0=s[0]][last state: 0, 1][how many times same: 0, 1]
    let mut dp: Vec<Vec<Vec<usize>>> = vec![vec![vec![INF; 2]; 2]; n];
    // 初期状態をどこから定義するか考察する
    dp[0][s[0]][0] = 0;
    dp[0][s[0]^1][0] = c[0];

    for i in 1..n {
        for j in 0..=1 {
            // すでに操作する必要がないとき
            if s[i] == j { // do nothing, no cost
                dp[i][j][0] = dp[i][j][0].min(dp[i-1][1^j][0]);
                dp[i][j][1] = dp[i][j][1].min(dp[i-1][1^j][1]);
                dp[i][j][1] = dp[i][j][1].min(dp[i-1][j][0]);
            } else {
                dp[i][j][0] = dp[i][j][0].min(dp[i-1][1^j][0] + c[i]);
                dp[i][j][1] = dp[i][j][1].min(dp[i-1][1^j][1] + c[i]);
                dp[i][j][1] = dp[i][j][1].min(dp[i-1][j][0] + c[i]);
            }
        }
    }
    let ans = dp[n-1][0][1].min(dp[n-1][1][1]);
    println!("{}", ans);
}