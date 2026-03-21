use proconio::input;
use petgraph::unionfind::UnionFind;
struct Edge {
    from: usize, to: usize, weight: i64
}

impl Edge {
    fn new(from: usize, to: usize, weight: i64) -> Self {
        Self{from: from-1, to: to-1, weight: weight}
    }
}

// クラスカル法で最小全域木を求める。
// 貪欲法
fn main() {
    input!{n: usize, m: usize}
    let mut edges: Vec<Edge> = Vec::new();
    for _ in 0..m {
        input!{a: usize, b: usize, c: i64}
        edges.push(Edge::new(a, b, c));
    }
    edges.sort_by_key(|edge| edge.weight);
    // 最大全域木の場合は逆
    // edges.sort_by(|a, b| b.weight.cmp(&a.weight));
    
    let mut uf = UnionFind::new(n);
    let mut ans: i64 = 0;
    for i in 0..m {
        if !uf.equiv(edges[i].from, edges[i].to) {
            ans += edges[i].weight;
            uf.union(edges[i].from, edges[i].to);
        }
    }

    println!("{}", ans);
}