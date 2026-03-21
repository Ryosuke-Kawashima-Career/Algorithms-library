use proconio::{input, marker::Chars};
// EducationalDP-F
// LCS(Longest Common Sequence): 最長共通部分列
fn main() {
    input!{s: Chars, t: Chars}
    let slen: usize = s.len();
    let tlen: usize = t.len();
    let mut dp: Vec<Vec<usize>> = vec![vec![0; tlen+1]; slen+1];

    // 1. 漸化式
    for i in 1..=slen {
        for j in 1..=tlen {
            dp[i][j] = dp[i][j].max(dp[i][j-1]);
            dp[i][j] = dp[i][j].max(dp[i-1][j]);
            // 文字列の一文字分が一致するか
            if s[i - 1] == t[j - 1] {
                dp[i][j] = dp[i][j].max(dp[i-1][j-1] + 1);
            } else {
                dp[i][j] = dp[i][j].max(dp[i-1][j-1]);
            }
        }
    }

    // 文字列を復元する。
    let mut ans: String = String::new();
    let mut row: usize = slen;
    let mut col: usize = tlen;
    // 2. dpの構成の1.の手順を逆から行う
    while row > 0 && col > 0 {
        // 1.の漸化式のif文も再現する
        if s[row-1] == t[col-1] {
            ans = format!("{}{}", s[row-1], ans);
            row -= 1;
            col -= 1;
            continue;
        }
        if dp[row][col] == dp[row-1][col] {
            row -= 1;
            continue;
        }
        if dp[row][col] == dp[row][col-1] {
            col -= 1;
            continue;
        }
        if dp[row][col] == dp[row-1][col-1] {
            row -= 1;
            col -= 1;
            continue;
        }
    }

    println!("{}", ans);
}