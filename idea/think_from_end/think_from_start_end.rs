use proconio::input;
const INF: i64 = 1 << 60;
// abc309Dが例題
// 左右から挟み撃ちする。
// 双方向から考える。
fn main() {
    input!{n: usize, m: usize}
    let mut graph: Vec<Vec<(usize, i64)>> = vec![vec![]; n];
    for _ in 0..m {
        input!{a: usize, b: usize, c: i64}
        graph[a-1].push((b-1, c));
        graph[b-1].push((a-1, c));
    }

    let dist_from_start: Vec<i64> = dijkstra(0, &graph);
    let dist_from_end: Vec<i64> = dijkstra(n-1, &graph);
    for k in 0..n {
        let dist: i64 = dist_from_start[k] + dist_from_end[k];
        println!("{}", dist);
    }
}

fn dijkstra(start: usize, graph: &Vec<Vec<(usize, i64)>>) -> Vec<i64> {
    let n: usize = graph.len();
    let mut dist: Vec<i64> = vec![INF; n];
    dist[start] = 0;
    let mut que = std::collections::VecDeque::new();
    que.push_back(start);

    while let Some(vertex) = que.pop_front() {
        for &(next, cost) in graph[vertex].iter() {
            if dist[next] > dist[vertex] + cost {
                dist[next] = dist[vertex] + cost;
                que.push_back(next);
            }
        }
    }

    return dist;
}