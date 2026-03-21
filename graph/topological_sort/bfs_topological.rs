use proconio::{input, marker::Usize1};
// Educational DP G
// longest path of DAG(Directed Acyclic Graph)
// トポロジカルソート(bfs, O(V+E)) -> DP
fn main() {
    input!{n: usize, m: usize, edges: [(Usize1, Usize1); m]}
    let mut graph: Vec<Vec<usize>> = vec![vec![]; n];
    for &(from, to) in edges.iter() {
        graph[from].push(to);
    }

    let order = bfs_topological_sort(&graph);
    let mut dp: Vec<usize> = vec![0; n];
    dp[order[0]] = 0;
    for i in 0..n {
        for &next in graph[order[i]].iter() {
            dp[next] = dp[next].max(dp[order[i]] + 1);
        }
    }
    let mut ans: usize = *dp.iter().max().unwrap();
    println!("{}", ans);
}

// 頂点vを終点とするパスの最大の長さの配列を返す(reverseの必要なし)
fn bfs_topological_sort(graph: &Vec<Vec<usize>>) -> Vec<usize> {
    let n: usize = graph.len();
    // 頂点の入次数を管理する
    let mut in_degrees: Vec<usize> = vec![0; n];
    for from in 0..n {
        for &to in graph[from].iter() {
            in_degrees[to] += 1;
        }
    }
    // 入次数が0つまり終点
    let mut zero_degrees = std::collections::VecDeque::new();
    for v in 0..n {
        if in_degrees[v] == 0 {
            zero_degrees.push_back(v);
        }
    }

    // トポロジカルソートの始点の順番を記録する
    let mut order: Vec<usize> = Vec::new();
    while let Some(v) = zero_degrees.pop_front() {
        order.push(v);
        for &next in graph[v].iter() {
            // edge(v -> next)を破壊する
            in_degrees[next] -= 1;
            if in_degrees[next] == 0 {
                zero_degrees.push_back(next);
            }
        }
    }

    return order;
}