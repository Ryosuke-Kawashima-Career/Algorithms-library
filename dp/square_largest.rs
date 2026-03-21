use proconio::input;
use std::collections::HashSet;
// abc311E
// h*wのマス目に(a, b)に穴をあける.穴のない正方形の区間の組み合わせを求める
// 1.「全ての穴のない正方形を数える」という問題を解きたい
// 2. これは「右下が(i,j)である穴のない正方形を数える」という部分問題を全てのマス(i,j)に対して解けば求められる
// 3. 部分問題をDPで計算しよう!
fn main() {
    input!{h: usize, w: usize, n: usize, ab: [(usize, usize); n]}
    let holes = detect_holes(&ab);
    // dp[i][j]:=(マス(i,j)を右下とする穴のない正方形のうち、一辺の長さnの最大値. 無いときは0)
    // 1. (i,j)を右下とする一辺の長さがnの正方形領域に穴が無い。
    // 2. (i,j-1)を右下とする一辺の長さがn-1の正方形領域に穴が無い。
    // 3. (i-1,j)を右下とする一辺の長さがn-1の正方形領域に穴が無い。
    // 4. (i-1,j-1)を右下とする一辺の長さがn-1の正方形領域に穴が無い。
    // 5. (i,j)は穴が無いマスである。
    let mut dp: Vec<Vec<usize>> = vec![vec![0; w+1]; h+1];
    let mut ans = 0;

    for i in 1..=h {
        for j in 1..=w {
            if !holes.contains(&(i, j)) {
                dp[i][j] = dp[i-1][j].min(dp[i][j-1]).min(dp[i-1][j-1]) + 1;
            }
            if dp[i][j] > 0 {
                ans += dp[i][j];
            }
        }
    }
    println!("{}", ans);
}

fn detect_holes(ab: &Vec<(usize, usize)>) -> HashSet<(usize, usize)> {
    let mut coord = HashSet::new();
    for &(a, b) in ab.iter() {
        coord.insert((a, b));
    }
    return coord;
}