use proconio::input;
// 典型22
// 配る遷移のDP
// 配るマスが探索されているかに注意!!!(-INFで初期化もあり)
fn main() {
    input!{n: usize, a: [usize; n-1], b: [usize; n-1]}
    let mut dp: Vec<i64> = vec![-1; n];
    dp[0] = 0;

    for i in 0..n-1 {
        // if visted -> update!!!
        if dp[i] != -1 {
            dp[a[i]-1] = dp[a[i]-1].max(dp[i]+100);
            dp[b[i]-1] = dp[b[i]-1].max(dp[i]+150);
        }
    }

    println!("{}", dp[n-1]);
}