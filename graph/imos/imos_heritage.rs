use proconio::{input, marker::Usize1};
// abc309E
// imos tree dp: 遅延評価
fn main() {
    input!{n: usize, m: usize, p: [Usize1; n-1], xy: [(Usize1, usize); m]}
    let mut tree: Vec<Vec<usize>> = vec![vec![]; n];
    for i in 0..n-1 {
        tree[p[i]].push(i+1);
    }
    let mut dp: Vec<usize> = vec![0; n];
    // 後で子孫に渡す計算を行う: 遅延評価
    for &(x, y) in xy.iter() {
        // 1: 自分の代
        // maxで上書き
        dp[x] = dp[x].max(y+1);
    }
    dfs(0, n, &tree, &mut dp);
    let mut ans = 0;
    for v in 0..n {
        if dp[v] >= 1 {
            ans += 1;
        }
    }
    println!("{}", ans);
}

fn dfs(v: usize, parent: usize, tree: &Vec<Vec<usize>>, dp: &mut Vec<usize>) {
    let heritage = dp[v].saturating_sub(1);
    for &next in tree[v].iter() {
        if next == parent {
            continue;
        }
        // maxで上書き
        dp[next] = dp[next].max(heritage);
        dfs(next, v, tree, dp);
    }
}