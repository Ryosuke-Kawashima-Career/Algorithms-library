use proconio::{input, marker::Usize1};
const INF: i64 = 1 << 60;
// 鉄則73: 距離のパラメータが二つ
// 距離Cを最小化し，道に植えてある木の数Dを最大化する
// graphの情報を拡張する
fn main() {
    input!{n: usize, m: usize, abcd: [(Usize1, Usize1, i64, usize); m]}
    let mut graph: Vec<Vec<(usize, i64, usize)>> = vec![vec![]; n];
    for &(a, b, c, d) in abcd.iter() {
        graph[a].push((b, c, d));
        graph[b].push((a, c, d));
    }
    let dist: Vec<(i64, usize)> = dijkstra(0, &graph);
    println!("{} {}", dist[n-1].0, dist[n-1].1);
}

fn dijkstra(start: usize, graph: &Vec<Vec<(usize, i64, usize)>>) -> Vec<(i64, usize)> {
    use std::collections::BinaryHeap;
    use std::cmp::Reverse;
    let n: usize = graph.len();
    // (distance, trees)
    let mut dist: Vec<(i64, usize)> = vec![(INF, 0); n];
    dist[start].0 = 0;
    // min dists from start
    let mut que = BinaryHeap::new();
    que.push((Reverse(0), start));

    // cur -> next
    while let Some((Reverse(dist_from_start), cur)) = que.pop() {
        for &(next, dist_next, tree) in graph[cur].iter() {
            if dist[next].0 > dist_next + dist_from_start {
                dist[next].0 = dist_next + dist_from_start;
                dist[next].1 = dist[cur].1 + tree;
                que.push((Reverse(dist[next].0), next));
            } else if dist[next].0 == dist_next + dist_from_start {
                dist[next].1 = (dist[next].1).max(dist[cur].1 + tree);
                que.push((Reverse(dist[next].0), next));
            }
        }
    }

    return dist;
}