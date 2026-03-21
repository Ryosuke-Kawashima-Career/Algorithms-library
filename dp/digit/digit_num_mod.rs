use proconio::{input, marker::Chars};
const MODINT: usize = 1_000_000_007;
const MOD: usize = 13;
// abc135d
// 制約Nがデカいときは桁ごとに考えてみる
// Q. X mod13 = 5 (X: ??2??5の?を数字で置き換える)
fn main() {
    input!{s: Chars}
    let n: usize = s.len();
    // cases = dp[digit][mod]
    let mut dp: Vec<Vec<usize>> = vec![vec![0; MOD]; n+1];
    dp[0][0] = 1;
    for i in 1..=n {
        for j in 0..MOD {
            if s[i-1] == '?' {
                for digit in 0..10 {
                    // j*10で一桁分，数数を大きくする
                    let mod_next = (j * 10 + digit) % MOD;
                    dp[i][mod_next] += dp[i-1][j];
                    dp[i][mod_next] %= MODINT;
                }
            } else {
                let digit = s[i-1] as usize - '0' as usize;
                let mod_next = (j * 10 + digit) % MOD;
                dp[i][mod_next] += dp[i-1][j];
                dp[i][mod_next] %= MODINT;
            }
        }
    }
    println!("{}", dp[n][5] % MODINT);
}