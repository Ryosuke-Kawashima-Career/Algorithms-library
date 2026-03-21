use proconio::{input, marker::Usize1};
use std::collections::HashSet;
use std::collections::VecDeque;
// ABC441D
// Q. keypoints:
// 各頂点の出次数は 4 以下
// 頂点0から頂点vへ向かうpathでちょうど L 回辺を通る。このとき、同じ辺を複数回通っても良いが、通るたびに回数にカウントされ，通った辺のコストの総和が S 以上 T 以下
// A. ただ単にBFSすればよい
fn main() {
    input! {n: usize, m: usize, l: usize, s: i64, t: i64, uvc: [(Usize1, Usize1, i64); m]}
    let mut graph: Vec<Vec<(usize, i64)>> = vec![vec![]; n];
    for (u, v, c) in uvc {
        graph[u].push((v, c));
    }
    let vertexes = bfs(0, &graph, l, s, t);
    for v in vertexes {
        print!("{} ", v + 1);
    }
    println!("");
}

fn bfs(start: usize, graph: &Vec<Vec<(usize, i64)>>, l: usize, s: i64, t: i64) -> Vec<usize> {
    let n: usize = graph.len();
    // (vertex, edge_count, cost)
    let mut visited: HashSet<(usize, usize, i64)> = HashSet::new();
    let mut queue: VecDeque<(usize, usize, i64)> = VecDeque::new();
    queue.push_back((start, 0, 0));
    let mut vertexes: Vec<usize> = vec![];
    while let Some((cur_v, edges, cost)) = queue.pop_front() {
        if edges > l {
            continue;
        }
        if cost > t {
            continue;
        }
        if visited.contains(&(cur_v, edges, cost)) {
            continue;
        }
        visited.insert((cur_v, edges, cost));
        if edges == l && s <= cost && cost <= t {
            vertexes.push(cur_v);
        }
        for &(next_v, next_cost) in graph[cur_v].iter() {
            queue.push_back((next_v, edges + 1, cost + next_cost));
        }
    }
    vertexes.sort();
    vertexes.dedup();
    vertexes
}
