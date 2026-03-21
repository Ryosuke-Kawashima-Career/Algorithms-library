use proconio::input;
use std::cmp::Reverse;
use std::collections::BinaryHeap;
// abc443D
// Q. Find the minimum number of operations to make all rows connected with the Euclidean distance = 1 among the neighboring columns.
// A. 最短路双対いわゆる牛ゲーなる典型問題に帰着させて超頂点からの距離を計算するらしい
// A1. 超頂点から各頂点iにR[i]の辺を張り、各頂点iから頂点i+1,i-1にコスト1の辺を張る
// A2. 超頂点からの最短路を計算する
fn main() {
    input! {t: usize}
    let mut answer: Vec<usize> = Vec::new();
    for _case in 0..t {
        let ans = solve();
        answer.push(ans);
    }
    for i in 0..t {
        println!("{}", answer[i]);
    }
}

fn solve() -> usize {
    input! {n: usize, rows: [i64; n]}
    // Node0: Super Vertex
    // Node1..=n: Vertex
    let mut graph: Vec<Vec<(usize, i64)>> = vec![vec![]; n + 1];
    for i in 1..=n {
        graph[0].push((i, rows[i - 1]));
        if i + 1 <= n {
            graph[i].push((i + 1, 1));
        }
        graph[i].push((i - 1, 1));
    }
    let dist_from_super_vertex: Vec<i64> = dijkstra(0, &graph);
    let mut cost: usize = 0;
    for i in 1..=n {
        cost += (rows[i - 1] - dist_from_super_vertex[i]) as usize;
    }
    cost
}

fn dijkstra(start: usize, graph: &Vec<Vec<(usize, i64)>>) -> Vec<i64> {
    let mut dist: Vec<i64> = vec![i64::MAX; graph.len()];
    let mut heap: BinaryHeap<(Reverse<i64>, usize)> = BinaryHeap::new();
    heap.push((Reverse(0), start));
    dist[start] = 0;
    while let Some((Reverse(dist_u), u)) = heap.pop() {
        if dist[u] != dist_u {
            continue;
        }
        for (v, cost) in &graph[u] {
            let new_dist = dist_u + cost;
            if new_dist < dist[*v] {
                dist[*v] = new_dist;
                heap.push((Reverse(new_dist), *v));
            }
        }
    }
    dist
}
