use proconio::{input, marker::Usize1};
// abc373d: weighted union-find: 相対座標
// A. dfsやbfsで代替可能
fn main() {
    input!{n: usize, m: usize, edges: [(Usize1, Usize1, i64); m]}
    let mut graph: Vec<Vec<(usize, i64)>> = vec![vec![]; n];
    for &(u, v, cost) in edges.iter() {
        graph[u].push((v, cost));
        // 反対方向のエッジを追加する!!
        graph[v].push((u, -cost));
    }
    let potentials: Vec<i64> = bfs(&graph);
    for v in 0..n {
        print!("{} ", potentials[v]);
    }
    println!("");
}

fn bfs(graph: &Vec<Vec<(usize, i64)>>) -> Vec<i64> {
    use std::collections::VecDeque;
    let n: usize = graph.len();
    let mut potentials: Vec<i64> = vec![0; n];
    let mut seen: Vec<bool> = vec![false; n];
    
    for v in 0..n {
        if seen[v] {
            continue;
        }
        let mut que = VecDeque::new();
        que.push_back(v);
        while let Some(cur) = que.pop_front() {
            for &(next, cost) in graph[cur].iter() {
                if !seen[next] {
                    potentials[next] = potentials[cur] + cost;
                    seen[next] = true;
                    que.push_back(next);
                }
            }
        }
    }

    return potentials;
}