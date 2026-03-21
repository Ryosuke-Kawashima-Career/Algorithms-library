use proconio::{input, marker::Usize1};
// Educational DP G
// Q. Given a acyclic graph, calculate its lengest path
// A. Memo DP
fn main() {
    input!{n: usize, m: usize, edges: [(Usize1, Usize1); m]}
    let mut graph: Vec<Vec<usize>> = vec![vec![]; n];
    for &(a, b) in edges.iter() {
        graph[a].push(b);
    }

    let mut dp: Vec<usize> = vec![0; n];
    let mut seen: Vec<bool> = vec![false; n];
    for v in 0..n {
        if !seen[v] {
            dfs(v, &graph, &mut dp, &mut seen);
        }
    }
    let ans = dp.iter().max().unwrap();
    println!("{}", ans);
}

fn dfs(v: usize, graph: &Vec<Vec<usize>>, dp: &mut Vec<usize>, seen: &mut Vec<bool>) -> usize {
    // return if already known
    if seen[v] {
        return dp[v];
    }
    seen[v] = true;
    let mut res: usize = 0;
    // calculate the maximum of the candidates
    for &next in graph[v].iter() {
        let sub: usize = dfs(next, graph, dp, seen);
        res = res.max(sub + 1);
    }

    // memo
    dp[v] = res;
    return res;
}