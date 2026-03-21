use proconio::input;

// 動的計画法によって最長パスを求める。edudp.G
// 辺の長さがすべて1
fn main() {
    input!{n: usize, m: usize}
    let mut graph: Vec<Vec<usize>> = vec![vec![]; n];
    // dp[i]: iからの最長距離
    let mut dp: Vec<i64> = vec![-1; n];
    for _ in 0..m {
        input!{x: usize, y: usize}
        graph[x-1].push(y-1);
    }

    // 深さ優先探索
    for i in 0..n {
        dfs(i, &graph, &mut dp);
    }

    let ans: i64 = *dp.iter().max().unwrap();
    println!("{}", ans);
}

fn dfs(start: usize, graph: &Vec<Vec<usize>>, dist: &mut Vec<i64>) {
    // 距離が計算されているなら if visited
    if dist[start] != -1 {
        return;
    }

    let mut res: i64 = 0;
    // 隣接点をすべて調べる。
    for &v in graph[start].iter() {
        // 帰りがけの再帰関数(子の情報を使う。)
        dfs(v, graph, dist);
        // 1は自分自身
        res = res.max(dist[v]+1);
    }

    dist[start] = res;
}