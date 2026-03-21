use proconio::input;
use proconio::marker::Chars;
const INF: usize = 1 << 60;
// 累積和を取る
// abc337D
// end = start + span: prefix[start + span] - prefix[start]
fn main() {
    input!{h: usize, w: usize, k: usize, s: [Chars; h]}
    let mut x_prefix_rows: Vec<Vec<usize>> = vec![vec![0; w+1]; h];
    let mut dot_prefix_rows: Vec<Vec<usize>> = vec![vec![0; w+1]; h];
    let mut x_prefix_cols: Vec<Vec<usize>> = vec![vec![0; w]; h+1];
    let mut dot_prefix_cols: Vec<Vec<usize>> = vec![vec![0; w]; h+1];
    let mut ans: usize = INF;
    
    // 行についてxと.の累積和を取る
    for i in 0..h {
        for j in 1..=w {
            x_prefix_rows[i][j] += x_prefix_rows[i][j-1];
            dot_prefix_rows[i][j] += dot_prefix_rows[i][j-1];
            if s[i][j-1] == 'x' {
                x_prefix_rows[i][j] += 1;
            }
            if s[i][j-1] == '.' {
                dot_prefix_rows[i][j] += 1;
            }
        }
    }
    for i in 0..h {
        for j in 0..=w {
            if j + k <= w {
                let x_num: usize = x_prefix_rows[i][j+k] - x_prefix_rows[i][j];
                let dot_num: usize = dot_prefix_rows[i][j+k] - dot_prefix_rows[i][j];
                if x_num == 0 {
                    ans = ans.min(dot_num);
                }
            }
        }
    }
    // 列についてxと.の累積和を取る
    for j in 0..w {
        for i in 1..=h {
            x_prefix_cols[i][j] += x_prefix_cols[i-1][j];
            dot_prefix_cols[i][j] += dot_prefix_cols[i-1][j];
            if s[i-1][j] == 'x' {
                x_prefix_cols[i][j] += 1;
            }
            if s[i-1][j] == '.' {
                dot_prefix_cols[i][j] += 1;
            }
        }
    }
    for j in 0..w {
        for i in 0..=h {
            if i + k <= h {
                let x_num: usize = x_prefix_cols[i+k][j] - x_prefix_cols[i][j];
                let dot_num: usize = dot_prefix_cols[i+k][j] - dot_prefix_cols[i][j];
                if x_num == 0 {
                    ans = ans.min(dot_num);
                }
            }
        }
    }
    if ans == INF {
        println!("-1");
    } else {
        println!("{}", ans);
    }
}