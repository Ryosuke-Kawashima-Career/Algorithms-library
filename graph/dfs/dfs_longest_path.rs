use proconio::{input, marker::Usize1};
// abc317C
fn main() {
    input!{n: usize, m: usize, edges: [(Usize1, Usize1, i64); m]}
    let mut graph: Vec<Vec<(usize, i64)>> = vec![vec![]; n];
    for &(a, b, c) in edges.iter() {
        graph[a].push((b, c));
        graph[b].push((a, c));
    }
    let mut seen: Vec<bool> = vec![false; n];
    let mut ans: i64 = 0;
    for v in 0..n {
        let dist = dfs(v, &graph, &mut seen);
        ans = ans.max(dist);
    }
    println!("{}", ans);
}

fn dfs(v: usize, graph: &Vec<Vec<(usize, i64)>>, seen: &mut Vec<bool>) -> i64 {
    let mut res: i64 = 0;
    let n: usize = graph.len();
    if seen[v] {
        return 0;
    }
    seen[v] = true;

    for &(next, cost) in graph[v].iter() {
        if seen[next] {
            continue;
        }
        let dist = dfs(next, graph, seen) + cost;
        // 帰りがけに最大値を更新する
        res = res.max(dist);
    }

    // 帰りがけにseenを復元する
    seen[v] = false;
    return res;
}