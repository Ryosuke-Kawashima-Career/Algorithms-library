use proconio::input;
const INF: i64 = 1 << 60;
// 典型アルゴリズム問題集F
// 最小全域木問題(Minimum Spanning Tree(MST))
// プリム法: O(ElogV)
fn main() {
    input!{n: usize, m: usize, edges: [(usize, usize, i64); m]}
    let mut graph: Vec<Vec<(usize, i64)>> = vec![vec![]; n];
    for &(u, v, cost) in edges.iter() {
        graph[u].push((v, cost));
        graph[v].push((u, cost));
    }
    let min_length = prim(0, &graph);
    println!("{}", min_length);
}

fn prim(start: usize, graph: &Vec<Vec<(usize, i64)>>) -> i64 {
    use std::collections::BinaryHeap;
    use std::cmp::Reverse;
    let mut min_length: i64 = 0;
    let n: usize = graph.len();
    let mut seen: Vec<bool> = vec![false; n];
    let mut que = BinaryHeap::new();
    // (shortest edge, cur vertex)
    que.push((Reverse(0), start));

    // 確定した頂点から不確定の頂点のエッジの中で最小のものを選ぶ
    while let Some((Reverse(min_cost), v)) = que.pop() {
        if seen[v] {
            continue;
        }
        seen[v] = true;

        min_length += min_cost;
        for &(next, cost) in graph[v].iter() {
            if !seen[next] {
                que.push((Reverse(cost), next));
            }
        }
    }

    return min_length;
}