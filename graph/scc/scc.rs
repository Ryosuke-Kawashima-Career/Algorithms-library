use proconio::input;
// 典型21
// scc(Strongly Connected Component, 強連結成分分解): 
// 頂点 x から頂点 y に向かうパス、
// 頂点 y から頂点 x に向かうパスが、どちらも存在
// 有向グラフについて有効
fn main() {
    input!{n: usize, m: usize}
    let mut graph: Vec<Vec<usize>> = vec![vec![]; n];
    let mut seen: Vec<bool> = vec![false; n];
    let mut order: Vec<usize> = Vec::new();
    // 辺の向きが反対のグラフについても計算する。
    let mut graph_rev: Vec<Vec<usize>> = vec![vec![]; n];
    let mut seen_rev: Vec<bool> = vec![false; n];

    for _ in 0..m {
        input!{a: usize, b: usize}
        graph[a-1].push(b-1);
        graph_rev[b-1].push(a-1);
    }

    // first dfs
    for v in 0..n {
        if !seen[v] {
            dfs(v, &graph, &mut seen, &mut order);
        }
    }
    // 観測された順を逆にして、さかのぼる。
    order.reverse();

    // reversed dfs
    let mut ans: usize = 0;
    for &v in order.iter() {
        if !seen_rev[v] {
            let mut cnt: usize = 0;
            dfs_count_group_size(v, &graph_rev, &mut seen_rev, &mut cnt);
            ans += cnt * (cnt-1) / 2;
        }
    }

    println!("{}", ans);
}

fn dfs(v: usize, graph: &Vec<Vec<usize>>, seen: &mut Vec<bool>, order: &mut Vec<usize>) {
    seen[v] = true;

    for &next in graph[v].iter() {
        if !seen[next] {
            dfs(next, graph, seen, order);
        }
    }

    // 帰りがけの順番を計算する。
    order.push(v);
}

fn dfs_count_group_size(v: usize, graph: &Vec<Vec<usize>>, seen: &mut Vec<bool>, cnt: &mut usize) {
    seen[v] = true;
    // 行きがけでsizeを計算する。
    *cnt += 1;

    for &next in graph[v].iter() {
        if !seen[next] {
            dfs_count_group_size(next, graph, seen, cnt);
        }
    }
}

// let cnt: usize = dfs_return_group_size(v, 0, &graph_rev, &mut seen_rev);
fn dfs_return_group_size(v: usize, mut size: usize, graph: &Vec<Vec<usize>>, seen: &mut Vec<bool>) -> usize {
    seen[v] = true;
    // 自分自身を含める。
    // 行きがけでグループの大きさを計算する。
    size += 1;

    for &next in graph[v].iter() {
        if seen[next] {
            continue;
        }
        // 連結部分の最大値がそのグループの要素数
        size = size.max(dfs_return_group_size(next, size, graph, seen));
    }

    return size;
}

// let size = dfs_rev(v, &graph_rev, &mut seen_rev);
fn dfs_rev(v: usize, graph: &Vec<Vec<usize>>, seen: &mut Vec<bool>) -> usize {
    let mut size: usize = 1;
    seen[v] = true;
    for &next in graph[v].iter() {
        if seen[next] {
            continue;
        }
        let sub_size: usize = dfs_rev(next, graph, seen);
        size += sub_size;
    }
    return size;
}