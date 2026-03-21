use proconio::input;
const INF: usize = 1 << 60;
// 鉄則B19 EducationalDpE
// 文字の制約で添え字を決定する。(問題を言い換える。)
// 価値を最大化する。 <=> 重さを最小化する。
fn main() {
    input!{n: usize, w: usize, wv: [(usize, usize); n]}
    let mut value_sum: usize = 0;
    for i in 0..n {
        value_sum += wv[i].1;
    }
    // min weight = dp[i: item][j: value]
    let mut dp: Vec<Vec<usize>> = vec![vec![INF; value_sum+1]; n+1];
    dp[0][0] = 0;
    for i in 1..=n {
        for j in 0..=value_sum {
            dp[i][j] = dp[i][j].min(dp[i-1][j]);
            if j >= wv[i-1].1 {
                dp[i][j] = dp[i][j].min(dp[i-1][j - wv[i-1].1] + wv[i-1].0);
            }
        }
    }

    let mut max_value: usize = 0;
    for value in 0..=value_sum {
        if dp[n][value] <= w {
            max_value = value;
        }
    }

    println!("{}", max_value);
}