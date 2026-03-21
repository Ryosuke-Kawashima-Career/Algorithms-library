use proconio::{input, marker::Usize1};
// abc311C
// Functional Graph: N頂点N辺の有向グラフであって、各頂点から丁度一つの辺が出ているグラフ
// 有向閉路: 始点と終点が同じ循環するパス
// scc(互いに行き来できる頂点の集合): サイクルは強連結成分である
fn main() {
    input!{n: usize, a: [Usize1; n]}
    let mut graph: Vec<Vec<usize>> = vec![vec![]; n];
    let mut graph_rev: Vec<Vec<usize>> = vec![vec![]; n];
    for v in 0..n {
        graph[v].push(a[v]);
        graph_rev[a[v]].push(v);
    }
    let mut seen = vec![false; n];
    let mut seen_rev = vec![false; n];
    let mut order: Vec<usize> = Vec::new();

    for v in 0..n {
        if !seen[v] {
            dfs(v, &graph, &mut seen, &mut order);
        }
    }
    order.reverse();

    for i in 0..n {
        if !seen_rev[order[i]] {
            let mut order_rev: Vec<usize> = Vec::new();
            dfs(order[i], &graph_rev, &mut seen_rev, &mut order_rev);
            if order_rev.len() > 1 {
                print_cycle(&order_rev);
                return;
            }
        }
    }
}

// topological sortの応用
fn dfs(v: usize, graph: &Vec<Vec<usize>>, seen: &mut Vec<bool>, order: &mut Vec<usize>) {
    seen[v] = true;
    for &next in graph[v].iter() {
        if !seen[next] {
            dfs(next, graph, seen, order);
        }
    }
    // 帰りがけに記録する
    order.push(v);
}

fn print_cycle(cycle: &Vec<usize>) {
    let k: usize = cycle.len();
    println!("{}", k);
    for i in 0..k {
        print!("{} ", cycle[i]+1);
    }
    println!("");
}