use proconio::{input, marker::Usize1};
const INF: i64 = 1 << 60;
// ABC441D
// Q. keypoints:
// 各頂点の出次数は 4 以下
// 頂点0から頂点vへ向かうpathでちょうど L 回辺を通る。このとき、同じ辺を複数回通っても良いが、通るたびに回数にカウントされ，通った辺のコストの総和が S 以上 T 以下
// A. ただ単にDFSすればよい
fn main() {
    input! {n: usize, m: usize, l: usize, s: i64, t: i64, uvc: [(Usize1, Usize1, i64); m]}
    let mut graph: Vec<Vec<(usize, i64)>> = vec![vec![]; n];
    for (u, v, c) in uvc {
        graph[u].push((v, c));
    }
    let mut vertexes: Vec<usize> = Vec::new();
    // (start, cost, traversed edges, edge_limit, cost_lower, cost_upper, graph, vertexes)
    dfs(0, 0, 0, l, s, t, &graph, &mut vertexes);
    vertexes.sort();
    vertexes.dedup();
    for &v in &vertexes {
        print!("{} ", v + 1);
    }
    println!("");
}

fn dfs(
    cur_v: usize,
    cur_cost: i64,
    num_edges: usize,
    edge_limit: usize,
    cost_lower: i64,
    cost_upper: i64,
    graph: &Vec<Vec<(usize, i64)>>,
    vertexes: &mut Vec<usize>,
) {
    let n: usize = graph.len();
    if num_edges == edge_limit {
        if cost_lower <= cur_cost && cur_cost <= cost_upper {
            vertexes.push(cur_v);
        }
        return;
    }

    for &(next_v, cost) in &graph[cur_v] {
        let next_cost: i64 = cur_cost + cost;
        let next_edges: usize = num_edges + 1;
        if next_cost <= cost_upper && next_edges <= edge_limit {
            dfs(
                next_v, next_cost, next_edges, edge_limit, cost_lower, cost_upper, graph, vertexes,
            );
        }
    }
}
