use proconio::{input, marker::Usize1};
use std::collections::HashSet;
// abc420E
// Q. it does not erase the edges of the graph. the colors of the nodes change.
// A. Only the Roots matter!!!
fn main() {
    input!{n: usize, q: usize}
    let mut colors: Vec<usize> = vec![0; n];
    let mut uf = UnionFind::new(n);
    let mut blacks_for_roots: Vec<usize> = vec![0; n];
    // 0: white 1: black
    for _ in 0..q {
        input!{query_type: usize}
        if query_type == 1 {
            input!{u: Usize1, v: Usize1}
            // Do not forget to update the number of black nodes!!!!!!!!!
            if !uf.equiv(u, v) {
                let blacks_for_u: usize = blacks_for_roots[uf.root(u)];
                let blacks_for_v: usize = blacks_for_roots[uf.root(v)];
                uf.union(u, v);
                blacks_for_roots[uf.root(u)] = blacks_for_u + blacks_for_v;
            }
        } else if  query_type == 2 {
            // reverse the color
            input!{v: Usize1}
            colors[v] ^= 1;
            if colors[v] == 1 {
                blacks_for_roots[uf.root(v)] += 1;
            } else {
                blacks_for_roots[uf.root(v)] -= 1;
            }
        } else {
            input!{v: Usize1}
            if blacks_for_roots[uf.root(v)] > 0 {
                println!("Yes");
            } else {
                println!("No");
            }
        }
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