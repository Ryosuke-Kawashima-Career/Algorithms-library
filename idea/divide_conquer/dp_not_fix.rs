use proconio::input;
const INF: usize = 1 << 60;
// abc099c
// 一気に2つ以上のことをしようとしない
// 分解して考える（シンプルな場合を考える）
// DPが別解になる場合がある
fn main() {
    input!{n: usize}
    // n円を6^x, 9^x円の効果で作るときの最小の枚数 = dp[n]
    let mut dp: Vec<usize> = vec![INF; n+1];
    dp[0] = 0;
    
    for sum in 1..=n {
        // 1円を使うとき
        dp[sum] = dp[sum].min(dp[sum-1] + 1);
        // 6^x円を使うとき
        let mut exp6: usize = 6;
        while sum >= exp6 {
            dp[sum] = dp[sum].min(dp[sum-exp6] + 1);
            exp6 *= 6;
        }
        // 9^x円を使うとき
        let mut exp9: usize = 9;
        while sum >= exp9 {
            dp[sum] = dp[sum].min(dp[sum-exp9] + 1);
            exp9 *= 9;
        }
    }

    println!("{}", dp[n]);
}