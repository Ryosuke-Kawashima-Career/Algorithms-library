use proconio::input;
const INF: i64 = 1 << 60;
// 典型C14
// dpの復元をdfsで行う。
fn main() {
    input!{n: usize, m: usize}
    let mut graph: Vec<Vec<(usize, i64)>> = vec![vec![]; n];
    for _ in 0..m {
        input!{a: usize, b: usize, c: i64}
        graph[a-1].push((b-1, c));
        graph[b-1].push((a-1, c));
    }
    let dist: Vec<i64> = dijkstra(0, &graph);
    let mut used: Vec<bool> = vec![false; n];
    dfs_recon(n-1, &graph, &dist, &mut used);
    let passed_nodes: usize = used.iter().filter(|&x| *x).count();

    println!("{}", passed_nodes);
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

fn dfs_recon(cur: usize, graph: &Vec<Vec<(usize, i64)>>, dist: &Vec<i64>, used: &mut Vec<bool>) {
    used[cur] = true;
    for &(next, cost) in graph[cur].iter() {
        if used[next] {
            continue;
        }
        if dist[next] + cost == dist[cur] {
            dfs_recon(next, graph, dist, used);
        }
    }
}