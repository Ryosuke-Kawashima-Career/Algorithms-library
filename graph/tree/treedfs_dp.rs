use proconio::input;
// educationalDP-P
// 木の数え上げ木DP
// 子供の情報で親の情報を更新する。
fn main() {
    input!{n: usize}
    let mut graph: Vec<Vec<usize>> = vec![vec![]; n];
    for _ in 1..n {
        input!{x: usize, y: usize}
        graph[x-1].push(y-1);
        graph[y-1].push(x-1);
    }
    // 木の色の塗り方 = dp[vertex][color] (0: white, 1: black)
    let mut dp: Vec<Vec<i64>> = vec![vec![0; 2]; n];

    dfs(0, n, &graph, &mut dp);
    let ans = dp[0][0] + dp[0][1];
    println!("{}", ans);
}

// v: 現在の頂点とparent: 親の頂点を関数に渡すことで逆流を防ぐ
fn dfs(v: usize, parent: usize, graph: &Vec<Vec<usize>>, dp: &mut Vec<Vec<i64>>) {
    dp[v][0] = 1;
    dp[v][1] = 1;

    for &next in graph[v].iter() {
        // 逆流を防ぐ。
        if next == parent {
            continue;
        }
        // 帰りがけ
        dfs(next, v, graph, dp);
        let mut white_num = dp[v][0];
        white_num *= dp[next][0] + dp[next][1];
        // 黒は連続しない。
        let mut black_num = dp[v][1];
        black_num *= dp[next][0];

        dp[v][0] = white_num;
        dp[v][1] = black_num;
    }
}