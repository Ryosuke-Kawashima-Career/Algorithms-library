use proconio::input;
// 鉄則B65
// 木DP: 帰りがけにrankを計算する
fn main() {
    input!{n: usize, t: usize, ab: [(usize, usize); n-1]}
    let mut graph: Vec<Vec<usize>> = vec![vec![]; n];
    for i in 0..n-1 {
        graph[ab[i].0 - 1].push(ab[i].1 - 1);
        graph[ab[i].1 - 1].push(ab[i].0 - 1);
    }

    let mut ranks: Vec<usize> = vec![0; n];
    dfs_rank(t-1, n, &graph, &mut ranks);
    for &rank in ranks.iter() {
        print!("{} ", rank);
    }
}

fn dfs_rank(v: usize, parent: usize, graph: &Vec<Vec<usize>>, ranks: &mut Vec<usize>) {
    // 考えられる階級の内，最大の値を格納する
    let mut rank: usize = 0;

    for &next in graph[v].iter() {
        // 逆流を防ぐ
        if next == parent {
            continue;
        }
        dfs_rank(next, v, graph, ranks);
        // 帰りがけにrankを計算する
        let sub_rank: usize = ranks[next];
        rank = rank.max(sub_rank + 1);
    }

    ranks[v] = rank;
}