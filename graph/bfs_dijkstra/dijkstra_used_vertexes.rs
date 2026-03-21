use proconio::{input, marker::Usize1};
const INF: i64 = 1 << 60;
// 鉄則C14
// Q. 差点1から交差点Nまで最短経路で移動したいです。彼が通る可能性のある交差点の数は、交差点1と交差点Nを含めていくつ?
// A. dist(start, v) + dist(v, end) == min_distance
fn main() {
    input!{n: usize, m: usize, abc: [(Usize1, Usize1, i64); m]}
    let mut graph: Vec<Vec<(usize, i64)>> = vec![vec![]; n];
    for &(a, b, cost) in abc.iter() {
        graph[a].push((b, cost));
        graph[b].push((a, cost));
    }
    let dist_from_start: Vec<i64> = dijkstra(0, &graph);
    let dist_from_end: Vec<i64> = dijkstra(n-1, &graph);
    let min_dist: i64 = dist_from_start[n-1];
    let mut ans: usize = 0;
    for v in 0..n {
        if dist_from_start[v] + dist_from_end[v] == min_dist {
            ans += 1;
        }
    }
    println!("{}", ans);
}

// priority que: (min_distance, vertex)
fn dijkstra(start: usize, graph: &Vec<Vec<(usize, i64)>>) -> Vec<i64> {
    use std::collections::BinaryHeap;
    use std::cmp::Reverse;

    let n: usize = graph.len();
    let mut dist: Vec<i64> = vec![INF; n];
    dist[start] = 0;
    let mut que = BinaryHeap::new();
    // (cur dist, cur vertex)
    que.push((Reverse(0), start));
    
    while let Some((Reverse(cur_cost), v)) = que.pop() {
        for &(next, cost) in graph[v].iter() {
            let next_cost = cur_cost + cost;
            if dist[next] > next_cost {
                dist[next] = next_cost;
                que.push((Reverse(next_cost), next));
            }
        }
    }

    return dist;
}