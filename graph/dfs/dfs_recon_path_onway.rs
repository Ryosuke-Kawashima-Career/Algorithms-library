use proconio::input;
// 鉄則B62(経路復元: 行きがけ)
// dfs: 全探索 -> seenを復元する, 一例のみの場合 -> 復元の必要なし
fn main() {
    input!{n: usize, m: usize}
    let mut graph: Vec<Vec<usize>> = vec![vec![]; n];
    for _ in 0..m {
        input!{a: usize, b: usize}
        graph[a-1].push(b-1);
        graph[b-1].push(a-1);
    }
    let mut path: Vec<usize> = Vec::new();
    let mut ans: Vec<usize> = Vec::new();
    let mut seen: Vec<bool> = vec![false; n];
    dfs(0, &graph, &mut seen, &mut path, &mut ans);
    for i in 0..ans.len() {
        print!("{} ", ans[i] + 1);
    }
}

fn dfs(v: usize, graph: &Vec<Vec<usize>>, seen: &mut Vec<bool>, path: &mut Vec<usize>, ans: &mut Vec<usize>) {
    let n: usize = graph.len();
    path.push(v);
    seen[v] = true;
    if v == n - 1 {
        // ansに答えをコピー
        *ans = path.clone();
        return;
    }

    for &next in graph[v].iter() {
        if seen[next] {
            continue;
        }
        dfs(next, graph, seen, path, ans);
    }
    // 帰りがけにデータを戻す
    path.pop();
}