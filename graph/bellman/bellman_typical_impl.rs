// 1. 最短距離が更新されなくなるか、|V| 回目の更新が終わるまで以下を繰り返す
    // 全ての辺に対して、「d[v] = min{ d[u] + ( u から v への距離 ) }」という更新式を利用して最短距離を更新する
    // １回のループごとに１つの「最短距離が決まった頂点」を確定させていくイメージ
// 2. |V-1| 回以内の更新で終了すれば負の閉路は存在しない。|V| 回まで更新が続けば負の閉路が存在する
fn bellman(start: usize, graph: &Vec<Vec<(usize, i64)>>) -> (Vec<i64>, bool) {
    let n: usize = graph.len();
    let mut dist: Vec<i64> = vec![INF; n];
    dist[start] = 0;
    let mut exist_negative_loop = false;
    
    for update in 0..n {
        let mut is_updated = false;
        // u -> (cost) -> v
        for u in 0..n {
            // edges
            for &(v, cost) in graph[u].iter() {
                if dist[u] != INF && dist[v] > dist[u] + cost {
                    dist[v] = dist[u] + cost;
                    is_updated = true;
                }
            }
        }
        if update == n - 1 && is_updated {
            exist_negative_loop = true;
        } 
    }

    return (dist, exist_negative_loop);
}