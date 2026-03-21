use proconio::input;
const MOD: usize = 1_000_000_007;
// educational DP O: Matching
// matching: N=21 -> O(N*2^N)
fn main() {
    input!{n: usize, a: [[usize; n]; n]}
    let bits = 1 << n;
    // combinations = dp[one group][used ones of the other group]
    let mut dp: Vec<Vec<usize>> = vec![vec![0; bits]; n+1];
    dp[0][0] = 1;
    for i in 1..=n {
        for bit in 0..bits {
            // i-1個のペアがすでにできているか?(計算量を減らす)
            if bit.count_ones() == i as u32 - 1 {
                for j in 0..n {
                    // jがまだペアに入っていないとき
                    if bit >> j & 1 == 0 && a[i-1][j] == 1 {
                        let next_state = bit | (1 << j);
                        dp[i][next_state] += dp[i-1][bit];
                        dp[i][next_state] %= MOD;
                    }
                }
            }
        }
    }
    println!("{}", dp[n][bits-1] % MOD);
}