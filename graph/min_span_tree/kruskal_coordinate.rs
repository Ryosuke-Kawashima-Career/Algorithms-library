use proconio::input;
use petgraph::unionfind::UnionFind;
// abc065d
// 最小全域木の辺の取り方を工夫する
// 全ての辺をx座標でソートすると、隣り合った頂点間の辺しか使われない。
// もし、違う頂点同士の辺を使うと、その間にある頂点が無駄に連結されないことになってしまう。
// 同様に全ての辺をy座標でソートして隣り合った頂点間の辺も利用する。
// 辺の大きさ: min(∣a−c∣,∣b−d∣)
fn main() {
    input!{n: usize, xy: [(i64, i64); n]}
    // x,y軸ごとに計算する
    let mut x: Vec<(usize, i64)> = Vec::new();
    let mut y: Vec<(usize, i64)> = Vec::new();
    for i in 0..n {
        x.push((i, xy[i].0));
        y.push((i, xy[i].1));
    }
    x.sort_by_key(|tup| tup.1);
    y.sort_by_key(|tup| tup.1);
    let mut edges: Vec<(usize, usize, i64)> = Vec::new();
    for i in 1..n {
        edges.push((x[i-1].0, x[i].0, x[i].1 - x[i-1].1));
        edges.push((y[i-1].0, y[i].0, y[i].1 - y[i-1].1));
    }
    edges.sort_by_key(|tup| tup.2);
    let mut uf = UnionFind::new(n);

    let mut ans = 0;
    for &(node1, node2, cost) in edges.iter() {
        if !uf.equiv(node1, node2) {
            ans += cost;
            uf.union(node1, node2);
        }
    }
    println!("{}", ans);
}