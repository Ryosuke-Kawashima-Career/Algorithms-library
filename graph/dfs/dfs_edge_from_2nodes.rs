use proconio::input;
// abc318d
// 全探索(ノードを２つずつ選び、エッジを選ぶ)
// O(n!!)
fn main() {
    input!{n: usize}
    // 隣接行列を使用する
    let mut graph: Vec<Vec<usize>> = vec![vec![0; n]; n];
    for i in 0..n {
        for j in (i+1)..n {
            input!{dij: usize}
            graph[i][j] = dij;
            graph[j][i] = dij;
        }
    }

    let mut ans: usize = 0;
    let mut used: Vec<bool> = vec![false; n];
    // ノードが偶数個か奇数個化で場合分けする
    if n % 2 == 0 {
        let size = dfs(0, &graph, &mut used);
        ans = ans.max(size);
    } else {
        // 使わないノードを全探索
        for i in 0..n {
            used[i] = true;
            let size = dfs(i, &graph, &mut used);
            ans = ans.max(size);
            used[i] = false;
        }
    }
    println!("{}", ans);
}

// ノードを２つずつ選ぶ(usedを復元しながら計算する)
fn dfs(v: usize, graph: &Vec<Vec<usize>>, used: &mut Vec<bool>) -> usize {
    if used.iter().all(|&x| x) {
        return 0;
    }
    let n: usize = used.len();
    let mut res: usize = 0;
    // 使われていないindexから始める
    let mut start: usize = n;
    for i in 0..n {
        if !used[i] {
            start = i;
            break; 
        }
    }
    if start == n {
        return 0;
    }

    used[start] = true;
    for next in start+1..n {
        if !used[next] {
            used[next] = true;
            res = res.max(graph[start][next] + dfs(next, graph, used));
            used[next] = false;
        }
    }
    used[start] = false;

    return res;
}