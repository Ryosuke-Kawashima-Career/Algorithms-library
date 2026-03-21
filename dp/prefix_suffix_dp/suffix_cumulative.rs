use proconio::{input, marker::Chars};
const INF: usize = 1 << 60;
// ABC442 F
// Q. Black(#) or White(.): 白のタイルを左上の隅に階段形状に配置する
// A. Prefix DP & Suffix Min DP (Cumulative Minimum)
fn main() {
    input! {n: usize, s: [Chars; n]}
    // dp[i][j]: min cost when from row=i, then all the cells below are black
    let mut dp: Vec<Vec<usize>> = vec![vec![0; n + 1]; n + 1];
    // Observe for each col
    let mut suffix_min_dp_cols_up: Vec<Vec<usize>> = vec![vec![INF; n + 1]; n + 1];
    for col in 1..=n {
        // Hypothesis: the initial cells are all #(Black)
        let mut cur_cost: usize = 0;
        // Vertial Rotation
        for row in 0..n {
            if s[row][col - 1] == '.' {
                cur_cost += 1;
            }
        }
        // See the previous column's dp values
        for row in (0..=n).rev() {
            if row == n {
                suffix_min_dp_cols_up[row][col] = dp[row][col - 1];
            } else {
                suffix_min_dp_cols_up[row][col] =
                    suffix_min_dp_cols_up[row + 1][col].min(dp[row][col - 1]);
            }
        }
        /* DP table's update */
        // Moving from h to h+1 means cell at row h flips from Black to White expectation.
        // If grid[h][j] is '.', error decreases (was wanted Black, now White match). -1
        // If grid[h][j] is '#', error increases (was wanted Black, now White mismatch). +1
        for row in 0..=n {
            dp[row][col] = suffix_min_dp_cols_up[row][col] + cur_cost;
            if row < n {
                if s[row][col - 1] == '.' {
                    cur_cost -= 1;
                } else {
                    cur_cost += 1;
                }
            }
        }
    }
    let mut min_dp: usize = INF;
    for row in 0..=n {
        min_dp = min_dp.min(dp[row][n]);
    }
    println!("{}", min_dp);
}
