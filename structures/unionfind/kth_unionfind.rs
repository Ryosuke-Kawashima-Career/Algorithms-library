use proconio::{input, marker::Usize1};
// abc372E: kth vertex of UnionFind
fn main() {
    input!{n: usize, q: usize}
    let mut uf = RankedUnionFind::new(n);
    for _ in 0..q {
        input!{query: usize}
        if query == 1 {
            input!{u: Usize1, v: Usize1}
            uf.union(u, v);
        } else {
            input!{v: Usize1, k: Usize1}
            let ans = uf.kth_vertex(v, k);
            if ans == -1 {
                println!("-1");
            } else {
                println!("{}", ans+1);
            }
        }
    }
}

use std::collections::BTreeSet;
// edit 1. struct 2. new::() 3. union()
pub struct RankedUnionFind {
    parent: Vec<usize>,
    // added for ranking
    members: Vec<BTreeSet<usize>>,
    size: Vec<usize>,
    rank: Vec<usize>,
}

impl RankedUnionFind {
    fn new(n: usize) -> Self {
        // added for ranking
        let mut members: Vec<BTreeSet<usize>> = vec![BTreeSet::new(); n];
        for v in 0..n {
            members[v].insert(v);
        }
        RankedUnionFind {
            parent: (0..n).collect(),
            members: members,
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
        // added for ranking
        let ry_members = self.members[ry].clone();
        self.members[rx].extend(ry_members.iter());
        true
    }

    // 0indexed
    pub fn kth_vertex(&mut self, v: usize, k: usize) -> i64 {
        let root_v = self.root(v);
        match self.members[root_v].iter().nth_back(k) {
            Some(&kth) => kth as i64,
            None => -1,
        }
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