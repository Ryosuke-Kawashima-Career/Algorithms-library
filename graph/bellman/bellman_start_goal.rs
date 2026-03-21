use proconio::{input, marker::Usize1};
const INF: i64 = 1 << 60;
// abc061D
// ベルマンフォード法: 単一始点最短経路問題を解ける
// 負の辺が含まれているような場合でも適用可能
// 負の閉路がグラフに含まれている際はそれを検出することができる
fn main() {
    input!{n: usize, m: i64, edges: [(Usize1, Usize1, i64); m]}
    let mut graph: Vec<Vec<(usize, i64)>> = vec![vec![]; n];
    for &(a, b, c) in edges.iter() {
        // 正負を反転する
        graph[a].push((b, -c));
    }
    let (dist, exist_negative_loop_start_goal) = bellman(0, n-1, &graph);

    if exist_negative_loop_start_goal {
        println!("inf");
    } else {
        // 符号の反転を戻す
        println!("{}", -dist[n-1]);
    }
}
// 1. 最短距離が更新されなくなるか、|V| 回目の更新が終わるまで以下を繰り返す
    // 全ての辺に対して、「d[v] = min{ d[u] + ( u から v への距離 ) }」という更新式を利用して最短距離を更新する
    // １回のループごとに１つの「最短距離が決まった頂点」を確定させていくイメージ
// 2. |V-1| 回以内の更新で終了すれば負の閉路は存在しない。|V| 回まで更新が続けば負の閉路が存在する
fn bellman(start: usize, goal: usize, graph: &Vec<Vec<(usize, i64)>>) -> (Vec<i64>, bool) {
    let n: usize = graph.len();
    let mut dist: Vec<i64> = vec![INF; n];
    dist[start] = 0;
    // startからgoalまでに負のサイクルがあるか?
    let mut exist_negative_loop_start_goal = false;
    
    for update in 0..n {
        // goalに関する道順が更新されたか?
        let mut is_updated_goal = false;
        // u -> (cost) -> v
        for u in 0..n {
            // edges
            for &(v, cost) in graph[u].iter() {
                if dist[u] != INF && dist[v] > dist[u] + cost {
                    dist[v] = dist[u] + cost;
                    if v == goal {
                        is_updated_goal = true;
                    }
                }
            }
        }
        if update == n - 1 && is_updated_goal {
            exist_negative_loop_start_goal = true;
        } 
    }

    return (dist, exist_negative_loop_start_goal);
}