use proconio::{input, marker::Usize1};
const MOD: usize = 1_000_000_007;
// educational DP P
// 木DP: 木の頂点を白黒で塗る(黒は隣り合わない)
fn main() {
    input!{n: usize, edges: [(Usize1, Usize1); n-1]}
    let mut graph: Vec<Vec<usize>> = vec![vec![]; n];
    for &(x, y) in edges.iter() {
        graph[x].push(y);
        graph[y].push(x);
    }
    let mut seen: Vec<bool> = vec![false; n];
    // dp[v: vertex][c: color(白:0, 黒:1)]
    let mut dp: Vec<Vec<usize>> = vec![vec![0; 2]; n];
    dfs(0, n, &graph, &mut dp, &mut seen);
    let ans: usize = dp[0][0] + dp[0][1];
    println!("{}", ans % MOD);
}

fn dfs(v: usize, parent: usize, graph: &Vec<Vec<usize>>, 
    dp: &mut Vec<Vec<usize>>, seen: &mut Vec<bool>) 
{
    seen[v] = true;
    dp[v][0] = 1;
    dp[v][1] = 1;
    for &next in graph[v].iter() {
        if next == parent || seen[next] {
            continue;
        }
        dfs(next, v, graph, dp, seen);
        // 帰りがけに計算する
        dp[v][0] *= (dp[next][0] + dp[next][1]) % MOD;
        dp[v][0] %= MOD;
        dp[v][1] *= dp[next][0];
        dp[v][1] %= MOD;
    }
}