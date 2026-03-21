use proconio::input;
use petgraph::unionfind::UnionFind;
// abc120d
// unionfindは切断クエリを実行できない
// 後ろからクエリを考える
// 余事象(非連結のノード <-> 連結ノードの組み合わせ)
// 逆転の発想: 分離によって不便さ(非連結のノードの組み合わせ)がどれだけ増えるかを考えるのではなく、
// 連結によって不便さがどれだけ減るかを考える
fn main() {
    input!{n: usize, m: usize, ab: [(usize, usize); m]}
    let mut uf = UnionFind::new(n);
    let mut answers = vec![0; m];
    let all_comb: usize = n * (n - 1) / 2;
    answers[m-1] = all_comb;
    // unionfindのグループの大きさ
    let mut sizes: Vec<usize> = vec![1; n];

    // クエリの反転
    for i in (1..m).rev() {
        let (u, v) = (ab[i].0-1, ab[i].1-1);
        if uf.equiv(u, v) {
            answers[i-1] = answers[i];
        } else {
            // u,vの親の数: rootから出す
            let reduced: usize = sizes[uf.find(u)] * sizes[uf.find(v)];
            let sum_size: usize = sizes[uf.find(u)] + sizes[uf.find(v)];
            answers[i-1] = answers[i] - reduced;
            uf.union(u, v);
            sizes[uf.find(u)] = sum_size;
        }
    }

    for i in 0..m {
        println!("{}", answers[i]);
    }
}