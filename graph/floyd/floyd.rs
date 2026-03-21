use proconio::input;
const INF: i64 = 1 << 60;
// GRL_1_C - 全点対間最短経路
// ワーシャルフロイド法(DP)
// すべての頂点間の距離が必要な時に使う。
// dp[i][i] < 0のとき負のループがある!!!
fn main() {
    input!{n: usize, m: usize}
    // 隣接行列
    let mut dp: Vec<Vec<i64>> = vec![vec![INF; n]; n];
    // 初期化する
    for i in 0..n {
        dp[i][i] = 0;
    }
    for _ in 0..m {
        input!{a: usize, b: usize, t: usize}
        dp[a-1][b-1] = t;
        dp[b-1][a-1] = t;
    }

    // order: k -> i -> j
    for k in 0..n {
        for i in 0..n {
            for j in 0..n {
                if dp[i][k] != INF && dp[k][j] != INF {
                    dp[i][j] = dp[i][j].min(dp[i][k]+dp[k][j]);
                }
            }
        }
    }

    let mut exist_negative_loop = false;
    for i in 0..n {
        if dp[i][i] < 0 {
            exist_negative_loop = true;
        }
    }
}