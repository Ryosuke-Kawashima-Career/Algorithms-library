use proconio::{input, marker::Chars};
const INF: i64 = 1 << 60;
// ABC 408 D
// Theme: transforming a "state manipulation" problem into a "maximum/minimum subarray sum" problem.
// Kadane's Algorithm (usually used for max subarray sum)
// Q. Change some 0s to 1s to minimize the number of 1s in the string.
// A. 目標が達成されているとき，最終的な S の形は，
// (0 が 0 個以上)(1 が 0 個以上)(0 が 0 個以上) という，3 つの状態からなる形で必ず書き表されます．
// dp[i][j]= (S の i 文字目までを書き換えて目標を達成する際，最後の文字が j 番目の状態に属しているとしたとき，書き換える必要のある文字数の最小値)
fn main() {
    input!{t: usize}
    for _case in 0..t {
        input!{n: usize, s: Chars}
        let mut dp: Vec<Vec<i64>> = vec![vec![INF as i64; 3]; n+1];
        dp[0][0] = 0;
        for i in 1..=n {
            for j in 0..=2 {
                // if the letter of s[i-1] is different from the state j, we need to flip it.
                let cost = if (j == 0 && s[i-1] == '1') || (j == 1 && s[i-1] == '0') || (j == 2 && s[i-1] == '1') {
                    1
                } else {
                    0
                };
                if j == 0 {
                    dp[i][j] = dp[i][j].min(dp[i-1][j] + cost);
                } else {
                    dp[i][j] = dp[i][j].min(dp[i-1][j] + cost);
                    // the state can be changed from j-1 to j.
                    dp[i][j] = dp[i][j].min(dp[i-1][j-1] + cost);
                }
            }
        }
        let ans = dp[n][2].min(dp[n][1]).min(dp[n][0]);
        println!("{}", ans);
    }
}