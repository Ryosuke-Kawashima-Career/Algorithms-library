use proconio::{input, marker::Usize1};
// 典型39
// 主客転倒テクニック: エッジの寄与率を計算する
fn main() {
    input!{n: usize, edges: [(Usize1, Usize1); n-1]}
    let mut tree: Vec<Vec<usize>> = vec![vec![]; n];
    for i in 0..n-1 {
        tree[edges[i].0].push(edges[i].1);
        tree[edges[i].1].push(edges[i].0);
    }
    let mut sizes: Vec<usize> = vec![0; n];
    dfs(0, n, &tree, &mut sizes);

    let mut ans: usize = 0;
    for &(u, v) in edges.iter() {
        let smaller_cluster = sizes[u].min(sizes[v]);
        ans += (n - smaller_cluster) * smaller_cluster;
    }
    println!("{}", ans);
}

fn dfs(v: usize, parent: usize, tree: &Vec<Vec<usize>>, sizes: &mut Vec<usize>) {
    sizes[v] = 1;
    for &next in tree[v].iter() {
        if next == parent {
            continue;
        }
        dfs(next, v, tree, sizes);
        // 帰りがけ
        let sub_size: usize = sizes[next];
        sizes[v] += sub_size;
    }
}