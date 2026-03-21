use proconio::{input, marker::Chars};
const INF: usize = 1 << 60;
// abc337D
// 区間クエリ: 累積和を取る
fn main() {
    input!{h: usize, w: usize, k: usize, s: [Chars; h]}
    // 行と列ごとに累積和を取る
    let mut prefix_row_o: Vec<Vec<usize>> = vec![vec![0; w+1]; h+1];
    let mut prefix_row_x: Vec<Vec<usize>> = vec![vec![0; w+1]; h+1];
    let mut prefix_col_o: Vec<Vec<usize>> = vec![vec![0; w+1]; h+1];
    let mut prefix_col_x: Vec<Vec<usize>> = vec![vec![0; w+1]; h+1];
    for i in 1..=h {
        for j in 1..=w {
            if s[i-1][j-1] == 'o' {
                prefix_row_o[i][j] = prefix_row_o[i][j-1] + 1;
            } else {
                prefix_row_o[i][j] = prefix_row_o[i][j-1];
            }
            if s[i-1][j-1] == 'x' {
                prefix_row_x[i][j] = prefix_row_x[i][j-1] + 1;
            } else {
                prefix_row_x[i][j] = prefix_row_x[i][j-1];
            }
        }
    }
    for j in 1..=w {
        for i in 1..=h {
            if s[i-1][j-1] == 'o' {
                prefix_col_o[i][j] = prefix_col_o[i-1][j] + 1;
            } else {
                prefix_col_o[i][j] = prefix_col_o[i-1][j];
            }
            if s[i-1][j-1] == 'x' {
                prefix_col_x[i][j] = prefix_col_x[i-1][j] + 1;
            } else {
                prefix_col_x[i][j] = prefix_col_x[i-1][j];
            }
        }
    }

    let mut min_cost: usize = INF;
    for i in 1..=h {
        for j in 1..=w {
            let end_i: usize = i + k - 1;
            let end_j: usize = j + k - 1;
            // xが列にないとき
            if end_i <= h && prefix_col_x[end_i][j] == prefix_col_x[i-1][j] {
                let cost_col = k - (prefix_col_o[end_i][j] - prefix_col_o[i-1][j]);
                min_cost = min_cost.min(cost_col);
            }
            if end_j <= w && prefix_row_x[i][end_j] == prefix_row_x[i][j-1] {
                let cost_row = k - (prefix_row_o[i][end_j] - prefix_row_o[i][j-1]);
                min_cost = min_cost.min(cost_row);
            }
        }
    }
    if min_cost == INF {
        println!("-1");
    } else {
        println!("{}", min_cost);
    }
}