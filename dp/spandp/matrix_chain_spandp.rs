// ALDS1_10_B: Matrix Chain
// 連鎖行列積問題とは、連鎖行列積の最小の掛け算の計算量を求める問題
// 連鎖行列積は行列積の順番によって掛け算の回数が変わる
// 1. 左の列数と右の行数が同じときのみ行列の掛け算ができる
// 2. 左の行数 × 左の列数=右の行数 × 右の列数 が行列の掛け算で生じる計算回数である
const INF: usize = 1 << 60;
fn main() {
    input!{n: usize, rc: [(usize, usize); n]}
    // 掛け算の数の最小値 = dp[l][r] [l r)の範囲
    let mut dp: Vec<Vec<usize>> = vec![vec![INF; n+1]; n+1];
    for i in 0..n {
        dp[i][i+1] = 0;
    }

    // span -> start -> end
    for span in 2..=n {
        for start in 0..=n {
            // [start end)なので end != start + span - 1
            let end: usize = start + span;
            if end > n {
                continue;
            }
            let row_l: usize = rc[start].0;
            // [start end)
            let col_r: usize = rc[end-1].1;
            // k: media!!!
            for k in start+1..end {
                // row_k == col_k-1!!!
                let row_k: usize = rc[k].0;
                let cost: usize = row_l * row_k * col_r;
                dp[start][end] = dp[start][end].min(dp[start][k] + dp[k][end] + cost);
            }
        }
    }

    println!("{}", dp[0][n]);
}