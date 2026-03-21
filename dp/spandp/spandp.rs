use proconio::input;
// 鉄則A21
// 区間DP
fn main() {
    input!{n: usize, orders: [(usize, i64); n]}
    // max point of [l r): dp[l][r]
    let mut dp: Vec<Vec<i64>> = vec![vec![0; n+1]; n+1];

    // 配る遷移
    for l in 0..n {
        // 逆順にfor文を回す。操作を l: -> <- : rのように外から内に行うから。
        for r in (l+1..=n).rev() {
            let mut value_l: i64 = dp[l][r];
            // dp[l][r]: [l r)
            // 区間[l r)に得点になるブロックが含まれているか？
            if l+1 <= orders[l].0-1 && orders[l].0-1 < r {
                value_l += orders[l].1;
            }
            let mut value_r: i64 = dp[l][r];
            if l <= orders[r-1].0-1 && orders[r-1].0-1 < r-1 {
                value_r += orders[r-1].1;
            }
            dp[l+1][r] = dp[l+1][r].max(value_l);
            dp[l][r-1] = dp[l][r-1].max(value_r);
        }
    }

    let mut max_value: i64 = 0;
    for i in 0..=n {
        for j in 0..=n {
            max_value = max_value.max(dp[i][j]);
        }
    }
    println!("{}", max_value);
}