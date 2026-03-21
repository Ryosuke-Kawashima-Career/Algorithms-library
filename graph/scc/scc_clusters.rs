use proconio::input;
// 典型21
// scc: strongly connected components(強連結成分分解)
// お互いに行き来できる頂点を同じグループになるように頂点をグループ分けする
fn main() {
    input!{n: usize, m: usize}
    let mut graph: Vec<Vec<usize>> = vec![vec![]; n];
    let mut graph_rev: Vec<Vec<usize>> = vec![vec![]; n];
    for _ in 0..m {
        input!{a: usize, b: usize}
        graph[a-1].push(b-1);
        // 辺の向きが反転したグラフ
        graph_rev[b-1].push(a-1);
    }
    // 帰りがけに記録する
    let mut order: Vec<usize> = Vec::new();
    let mut seen: Vec<bool> = vec![false; n];
    for v in 0..n {
        if seen[v] {
            continue;
        }
        dfs(v, &graph, &mut seen, &mut order);
    }
    // 反転する
    order.reverse();

    let mut seen_rev: Vec<bool> = vec![false; n];
    let mut ans: usize = 0;
    for &v in order.iter() {
        let size = dfs_rev(v, &graph_rev, &mut seen_rev);
        if size > 0 {
            ans += size * (size - 1) / 2;
        }
    }
    println!("{}", ans);
}

fn dfs(v: usize, graph: &Vec<Vec<usize>>, seen: &mut Vec<bool>, order: &mut Vec<usize>) {
    seen[v] = true;
    for &next in graph[v].iter() {
        if seen[next] {
            continue;
        }
        dfs(next, graph, seen, order);
    }
    // 帰りがけに記録する
    order.push(v);
}

fn dfs_rev(v: usize, graph: &Vec<Vec<usize>>, seen: &mut Vec<bool>) -> usize {
    let mut size: usize = 1;
    seen[v] = true;
    for &next in graph[v].iter() {
        if seen[next] {
            continue;
        }
        let sub_size: usize = dfs_rev(next, graph, seen);
        // 帰りがけにクラスターの大きさを記録する
        size += sub_size;
    }
    return size;
}