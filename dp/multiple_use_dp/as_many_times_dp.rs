const INF: usize = 1 << 60;
// Aizu online DPL_1_A
// Index is not necessary for the DP since we can use coins as many times as we want.
fn main() {
    let stdin = std::io::stdin();
    let mut sc = Scanner::new(stdin.lock());
    input! {sc = sc, n: usize, m: usize, c: [usize; m]}
    // dp[price] := ちょうど price を支払う最小の硬貨の枚数
    let mut dp = vec![INF; n + 1];
    dp[0] = 0;

    for coin in c {
        for price in coin..=n {
            if dp[price - coin] != INF {
                // Updates are cumulated!!!!!!!!
                dp[price] = dp[price].min(dp[price - coin] + 1);
            }
        }
    }

    let ans = dp[n];
    println!("{}", ans);
}
