use proconio::input;
// abc306D
// Q. x: 0=解毒剤, 1=毒, y=得点: 毒を2回食べると死ぬ, 解毒剤で元通り
// 考察: pairing, クエリの先読みと反転?
// A. DPかよ
fn main() {
    input!{n: usize, xy: [(usize, i64); n]}
    // 最大値 = dp[item][j: 毒の量(2で死ぬ)]
    let mut dp: Vec<Vec<i64>> = vec![vec![0; 2]; n+1];
    for i in 1..=n {
        let (dish_type, point) = xy[i-1];
        if dish_type == 0 {
            // 飲まない
            dp[i][0] = dp[i-1][0];
            // 解毒剤を飲む場合
            dp[i][0] = dp[i][0].max(dp[i-1][0] + point).max(dp[i-1][1] + point);
        } else {
            // 飲まない
            dp[i][0] = dp[i-1][0];
            dp[i][1] = dp[i-1][1];
            // 毒を飲む
            dp[i][1] = dp[i][1].max(dp[i-1][0] + point);
        }
    }
    // 下限値は0
    let ans = 0.max(dp[n][0]).max(dp[n][1]);
    println!("{}", ans);
}