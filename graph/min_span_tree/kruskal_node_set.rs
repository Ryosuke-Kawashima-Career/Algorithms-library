use proconio::{input, marker::Usize1};
// abc352E
// 以下を満たす辺e=(u,v)が存在する場合、辺eを候補から消してしまっても最小全域木に含まれる辺の重みの総和は変化しない。
// 要らない辺: 頂点uと頂点vを結ぶパスであって、辺eの重み以下の辺のみで構成され、かつ辺eを含まないようなものが存在する。
fn main() {
    input!{n: usize, m: usize}
    let mut min_length: i64 = 0;
    let mut uf = UnionFind::new(n);
    let mut edges: Vec<(i64, usize, usize)> = Vec::new();
    for _ in 0..m {
        // a: 頂点の集合
        // 貪欲法: a[0]とa[i]の辺をつなげばいい
        input!{k: usize, c: i64, a: [Usize1; k]}
        for i in 1..k {
            edges.push((c, a[0], a[i]));
        }
    }
    edges.sort();
    for &(cost, u, v) in edges.iter() {
        if !uf.equiv(u, v) {
            min_length += cost;
            uf.union(u, v);
        }
    }
    if !uf.is_linked() {
        println!("-1");
    } else {
        println!("{}", min_length);
    }
}

pub struct UnionFind {
    parent: Vec<usize>,
    size: Vec<usize>,
    rank: Vec<usize>,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        UnionFind {
            parent: (0..n).collect(),
            size: vec![1; n],
            rank: vec![0; n],
        }
    }

    // 根を返却
    pub fn root(&mut self, x: usize) -> usize {
        // parentが自分自身の場合は根
        if self.parent[x] == x {
            return x;
        }
        // 経路圧縮
        self.parent[x] = self.root(self.parent[x]);
        self.parent[x]
    }

    // xとyが同じ根か判定
    pub fn equiv(&mut self, x: usize, y: usize) -> bool {
        self.root(x) == self.root(y)
    }

    // xとyを合体
    pub fn union(&mut self, x: usize, y: usize) -> bool {
        let mut rx = self.root(x);
        let mut ry = self.root(y);
        // 既に同じ
        if rx == ry {
            return false;
        }

        // ryのrankが小さくなるように調整
        // ここを省略するとrxが親になる
        if self.rank[rx] < self.rank[ry] {
            std::mem::swap(&mut rx, &mut ry);
        }
        // ryの根をrxにする
        self.parent[ry] = rx;
        // rxのrank調整
        if self.rank[rx] == self.rank[ry] {
            self.rank[rx] += 1;
        }
        self.size[rx] += self.size[ry];
        true
    }

    // xのグループの要素数
    pub fn size(&mut self, x: usize) -> usize {
        let root = self.root(x);
        self.size[root]
    }

    // 連結かどうかを返却する
    pub fn is_linked(&mut self) -> bool {
        self.size(0) == self.size.len()
    }

    // グループの数を計算
    pub fn groups(&mut self) -> usize {
        let mut res: usize = 0;
        for x in 0..self.size.len() {
            // グループの数 = 根の数
            if x == self.root(x) {
                res += 1;
            }
        }
        res
    }
}