use proconio::{input, marker::Usize1};
const INF: usize = 1 << 60;
// abc317C: 巡回しないセールスマン
fn main() {
    input!{n: usize, m: usize, edges: [(Usize1, Usize1, usize); m]}
    let mut graph: Vec<Vec<(usize, usize)>> = vec![vec![]; n];
    for &(a, b, c) in edges.iter() {
        graph[a].push((b, c));
        graph[b].push((a, c));
    }
    let bits = 1 << n;
    // max length = dp[cur node][visited]
    let mut dp: Vec<Vec<usize>> = vec![vec![0; bits]; n];
    let mut ans = 0;
    // bit -> cur_v -> next_v
    for bit in 0..bits {
        for v in 0..n {
            for &(next, cost) in graph[v].iter() {
                // cur_vについてbit計算をし，巡回を防ぐ
                if v != next && bit >> v & 1 == 1 && bit >> next & 1 == 0 {
                    let next_state = bit | (1 << next);
                    dp[next][next_state] = dp[next][next_state].max(dp[v][bit] + cost);
                    ans = ans.max(dp[next][next_state]);
                } 
            }
        }
    }
    println!("{}", ans);
}