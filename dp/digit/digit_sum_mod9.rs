use proconio::input;
const MOD: usize = 1_000_000_007;
// 典型42
// 9で割った数は各位の和が9の倍数
fn main() {
    input!{k: usize}
    if k % 9 != 0 {
        println!("0");
        return;
    }
    // 各位の和がi = dp[i]
    let mut dp: Vec<usize> = vec![0; k+1];
    dp[0] = 1;
    for i in 1..=k {
        // 上の位を一つずつ増やす。
        for j in 1..=9 {
            if i >= j {
                dp[i] += dp[i-j] % MOD;
                dp[i] %= MOD;
            }
        }
    }

    println!("{}", dp[k]%MOD);
}