use proconio::{input, marker::Usize1};
// Educational DP G
// longest path of DAG(Directed Acyclic Graph)
// メモ化再帰
fn main() {
    input!{n: usize, m: usize, edges: [(Usize1, Usize1); m]}
    let mut graph: Vec<Vec<usize>> = vec![vec![]; n];
    for &(from, to) in edges.iter() {
        graph[from].push(to);
    }
    // -1: not visited
    let mut dp: Vec<i64> = vec![-1; n];
    let mut longest_path = 0;
    for v in 0..n {
        let path = memo_rec(v, &graph, &mut dp);
        longest_path = longest_path.max(path);
    }
    println!("{}", longest_path);
}

fn memo_rec(v: usize, graph: &Vec<Vec<usize>>, dp: &mut Vec<i64>) -> i64 {
    // if updated return
    if dp[v] != -1 {
        return dp[v];
    }
    let mut path: i64 = 0;
    for &next in graph[v].iter() {
        let sub_path = memo_rec(next, graph, dp);
        // 帰りがけに計算する
        path = path.max(sub_path + 1);
    }
    // memo and return
    dp[v] = path;
    return path;
}