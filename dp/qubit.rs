use proconio::{input, marker::Chars};
const MOD: usize = 1_000_000_007;
// abc104D
// Q. A??Cの文字列の?の文字をA,B,Cのいずれかに変えてできる部分列ABCの数
fn main() {
    input!{s: Chars}
    let n: usize = s.len();
    // cases = dp[item][last letter(qubit, A, B, C)]
    let mut dp: Vec<Vec<usize>> = vec![vec![0; 4]; n+1];
    dp[0][0] += 1;

    for i in 1..=n {
        if s[i-1] == '?' {
            // qubitはA,B,Cの3つの状態を同時に兼ねる
            dp[i][0] = 3 * dp[i-1][0];
            dp[i][0] %= MOD;
            for c in 1..4 {
                // not use + use
                dp[i][c] = 3 * dp[i-1][c] + dp[i-1][c-1];
                dp[i][c] %= MOD;
            }
        } else {
            for c in 0..4 {
                // not used -> copy
                dp[i][c] = dp[i-1][c];
            }
            let c_cur = to_num(s[i-1]);
            dp[i][c_cur] += dp[i-1][c_cur-1];
            dp[i][c_cur] %= MOD;
        }
    }

    println!("{}", dp[n][3] % MOD);
}

fn to_num(c: char) -> usize {
    return c as usize - 'A' as usize + 1;
}