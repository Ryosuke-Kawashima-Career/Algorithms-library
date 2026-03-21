use proconio::input;
// abc138d
// 木に対してimos法を使う。遅延評価
// あらかじめ値を用意してから、後でまとめて全体の計算をする。
// 終点がないので通常のimos法と違い、終点の-valは不要
fn main() {
    input!{n: usize, q: usize}
    let mut tree: Vec<Vec<usize>> = vec![vec![]; n];
    for _ in 1..n {
        input!{a: usize, b: usize}
        tree[a-1].push(b-1);
        tree[b-1].push(a-1);
    }
    let mut count: Vec<usize> = vec![0; n];

    for _ in 0..q {
        input!{parent: usize, x: usize}
        count[parent-1] += x;
    }
    // imos法をdfsで書くことで親の情報を子供に伝える。
    imos_dfs(0, n, &tree, &mut count);

    for i in 0..n {
        print!("{} ", count[i]);
    }
}

fn imos_dfs(v: usize, parent: usize, tree: &Vec<Vec<usize>>, count: &mut Vec<usize>) {
    let heritage: usize = count[v];

    for &next in tree[v].iter() {
        if next != parent {
            // 親の情報を子供に伝えるので行きがけ
            count[next] += heritage;
            imos_dfs(next, v, tree, count);
        }
    }
}