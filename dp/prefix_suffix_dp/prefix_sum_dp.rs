use proconio::input;
const MOD: usize = 1_000_000_007;
// Educational DP M
// 累積和で遷移を高速化する(DP高速化を見据える場合は、集めるDPにするといい)
// 場合の数の計算に累積和は有用である
fn main() {
    input!{n: usize, k: usize, a: [usize; n]}
    // dp[item][candy]
    let mut dp: Vec<Vec<usize>> = vec![vec![0; k+1]; n+1];
    dp[0][0] = 1;
    
    for i in 1..=n {
        let mut prefix: Vec<usize> = vec![0; k+2];
        for j in 1..=k+1 {
            prefix[j] = prefix[j-1] + dp[i-1][j-1];
            prefix[j] %= MOD;
        }
        // prefix[j] = dp[i-1][0] +...+ dp[i-1][j-1]
        for j in 0..=k {
            let min_index: usize = 0.max(j as i64 - a[i-1] as i64) as usize;
            dp[i][j] = prefix[j+1] + MOD - prefix[min_index];
            dp[i][j] %= MOD;
        }
    }
    println!("{}", dp[n][k] % MOD);
}