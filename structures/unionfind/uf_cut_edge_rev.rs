use proconio::input;
use petgraph::unionfind::UnionFind;
// 鉄則B66
// クエリを予めすべて取得して，逆順に処理する
// UnionFindは切断操作ができないので，Queryを後ろから考える
fn main() {
    input!{n: usize, m: usize, ab: [(usize, usize); m], q: usize}
    let mut queries: Vec<(usize, usize, usize)> = Vec::new();
    let mut is_suspended: Vec<bool> = vec![false; m];
    for _ in 0..q {
        input!{query: usize}
        if query == 1 {
            input!{x: usize}
            is_suspended[x-1] = true;
            queries.push((1, x-1, n));
        } else {
            input!{u: usize, v: usize}
            queries.push((2, u-1, v-1));
        }
    }
    let mut uf = UnionFind::new(n);
    // 切断されないエッジを接続する
    for edge in 0..m {
        if !is_suspended[edge] {
            uf.union(ab[edge].0-1, ab[edge].1-1);
        }
    }

    let mut answers: Vec<&str> = Vec::new();
    // クエリを後ろから考える
    for &(query, u, v) in queries.iter().rev() {
        if query == 1 {
            uf.union(ab[u].0-1, ab[u].1-1);
        } else {
            if uf.equiv(u, v) {
                answers.push("Yes");
            } else {
                answers.push("No");
            }
        }
    }

    // クエリ操作の向きを戻す
    answers.reverse();
    for ans in answers.iter() {
        println!("{}", ans);
    }
}