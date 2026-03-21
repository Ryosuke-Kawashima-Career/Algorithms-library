use proconio::{input, marker::Chars};
// abc365d: Janken
// Q. Takahashi never lost to Aoki.
// For i=1,2,…,N−1, Takahashi's move in the i-th game is different from his move in the (i+1)-th game.
// what is the maximum number of Takahashi's winning games
// A. dp  
fn main() {
    input!{n: usize, s: Chars}
    let to_num = |c: char| -> usize {
        match c {
            'R' => 0,
            'S' => 1,
            _ => 2,
        }
    };
    let mut dp: Vec<Vec<usize>> = vec![vec![0; 3]; n+1];
    for i in 1..=n {
        for j in 0..3 {
            let hand_aoki: usize = to_num(s[i-1]);
            // Takahashi never lose to aoki
            if hand_aoki == (j + 1) % 3 {
                dp[i][j] = dp[i][j].max(dp[i-1][(j+1)%3] + 1);
                dp[i][j] = dp[i][j].max(dp[i-1][(j+2)%3] + 1);
            } else if hand_aoki == j {
                dp[i][j] = dp[i][j].max(dp[i-1][(j+1)%3]);
                dp[i][j] = dp[i][j].max(dp[i-1][(j+2)%3]);
            }
        }
    }
    let ans: usize = dp[n][0].max(dp[n][1]).max(dp[n][2]);
    println!("{}", ans);
}