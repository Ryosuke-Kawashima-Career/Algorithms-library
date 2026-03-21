use proconio::{input, marker::Chars};
const MOD: usize = 1_000_000_007;
// educational DP S
// 桁DP: 配る形式の方が書きやすい
// dp[i][j][f: 0 or 1]=(上からi桁目まで決めて、Kより小さいことが確定して(f? いて:いなくて)、条件jを満たすようなもの)
// 1以上k以下の数字でdの倍数の数
fn main() {
    // 数字を文字列で受け取る
    input!{k: Chars, d: usize}
    // 数字ではなく数字の桁に注目する
    let digits: Vec<usize> = k.iter().map(|&x| x as usize - '0' as usize).collect();
    let n: usize = digits.len();
    // dp[i: item][j: mod of sum][0(no), 1(yes): is less than k]
    let mut dp: Vec<Vec<Vec<usize>>> = vec![vec![vec![0; 2]; d]; n+1];
    dp[0][0][0] = 1;
    for i in 0..n {
        for j in 0..d {
            for is_less in 0..2 {
                for digit in 0..10 {
                    // 数字がkより小さいことがすでに確定またはこれで確定
                    if digit < digits[i] {
                        dp[i+1][(j+digit) % d][1] += dp[i][j][is_less];
                        dp[i+1][(j+digit) % d][1] %= MOD;
                    } else if digit == digits[i] { // 確定について現状維持
                        dp[i+1][(j+digit) % d][is_less] += dp[i][j][is_less];
                        dp[i+1][(j+digit) % d][is_less] %= MOD;
                    } else {
                        // すでに確定しているとき
                        if is_less == 1 {
                            dp[i+1][(j+digit) % d][is_less] += dp[i][j][is_less];
                            dp[i+1][(j+digit) % d][is_less] %= MOD;
                        }
                    }
                }
            }
        }
    }

    // 答えから0の分を-1する
    let ans = (dp[n][0][0] + dp[n][0][1]) % MOD + (MOD - 1);
    println!("{}", ans % MOD);
}