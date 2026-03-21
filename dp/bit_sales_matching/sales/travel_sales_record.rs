use proconio::input;
const INF: i64 = 1 << 60;
// square869120G: 場合の数を記録する巡回セールスマン問題
fn main() {
    input!{n: usize, m: usize, edges: [(usize, usize, i64, i64); m]}
    // 隣接リストを作成する
    let mut graph: Vec<Vec<(usize, i64, i64)>> = vec![vec![]; n];
    for &(u, v, cost, limit) in edges.iter() {
        graph[u-1].push((v-1, cost, limit));
        graph[v-1].push((u-1, cost, limit));
    }
    let bits: usize = 1 << n;
    // (最短距離, 場合の数) = dp[i: cur_pos][j: visited]
    let mut dp: Vec<Vec<(i64, usize)>> = vec![vec![(INF, 0); bits]; n];
    // 0を始点としてよい
    dp[0][0] = (0, 1);
    // state -> node
    for bit in 0..bits {
        for i in 0..n {
            for &(next, cost, limit) in graph[i].iter() {
                // if next is not visited
                if bit >> next & 1 == 0 {
                    let next_state: usize = bit | (1 << next);
                    let next_time: i64 = dp[i][bit].0 + cost;
                    if next_time <= limit {
                        if dp[next][next_state].0 > next_time {
                            dp[next][next_state].0 = next_time;
                            dp[next][next_state].1 = dp[i][bit].1;
                        } else if dp[next][next_state].0 == next_time {
                            dp[next][next_state].1 += dp[i][bit].1;
                        }
                    }
                }
            }
        }
    }

    if dp[0][bits-1].0 == INF {
        println!("IMPOSSIBLE");
    } else {
        println!("{} {}", dp[0][bits-1].0, dp[0][bits-1].1);
    }
}