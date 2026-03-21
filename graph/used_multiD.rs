use proconio::{input, marker::Chars};
use std::collections::{HashMap, HashSet};
const N1: usize = 1usize.wrapping_neg();
const D4: [(usize, usize); 4] = [
    (N1, 0),
    (0, N1), (0, 1),
    (1, 0)
];
// abc351D: '.'=通過可能, '#'=通過不能, 'x'=動けなくなる
// usedを三次元にする: used[y][x][group(root)]
// x # x . .
// . x . . x
// x # x x #
fn main() {
    input!{h: usize, w: usize, mut s: [Chars; h]}
    // '#'から一マス分'x'を拡張する
    expand(&mut s);
    let max_size: usize = max_cluster(&s);

    println!("{}", max_size);
}

fn expand(graph: &mut Vec<Vec<char>>) {
    let h: usize = graph.len();
    let w: usize = graph[0].len();
    for y in 0..h {
        for x in 0..w {
            if graph[y][x] == '#' {
                for &(dy, dx) in D4.iter() {
                    let next_y: usize = y.wrapping_add(dy);
                    let next_x: usize = x.wrapping_add(dx);
                    if next_y < h && next_x < w {
                        if graph[next_y][next_x] == '.' {
                            graph[next_y][next_x] = 'x';
                        }
                    }
                }
            }
        }
    }
}

fn max_cluster(graph: &Vec<Vec<char>>) -> usize {
    let h: usize = graph.len();
    let w: usize = graph[0].len();
    let mut sizes: Vec<Vec<usize>> = vec![vec![0; w]; h];
    // '.'のクラスターを計算する
    let mut uf = UnionFind::new(h*w);
    for y in 0..h {
        for x in 0..w {
            if graph[y][x] == '#' {
                continue;
            }
            if graph[y][x] == 'x' {
                sizes[y][x] = 1;
                continue;
            }
            let node: usize = y * w + x;
            for &(dy, dx) in D4.iter() {
                let next_y: usize = y.wrapping_add(dy);
                let next_x: usize = x.wrapping_add(dx);
                if next_y < h && next_x < w {
                    if graph[next_y][next_x] == '.' {
                        let next_node: usize = next_y * w + next_x;
                        uf.union(node, next_node);
                    }
                }
            }
        }
    }

    for y in 0..h {
        for x in 0..w {
            let node: usize = y * w + x;
            sizes[y][x] = uf.size(node);
        }
    }

    let mut map_size = HashMap::new();
    for y in 0..h {
        for x in 0..w {
            let node = y * w + x;
            map_size.insert(uf.root(node), uf.size(node));
        }
    }

    // 一回だけ通れる'x'の分を足す
    let mut used: Vec<Vec<HashSet<usize>>> = vec![vec![HashSet::new(); w]; h];
    for y in 0..h {
        for x in 0..w {
            if graph[y][x] != '.' {
                continue;
            }
            let node = y * w + x;
            for &(dy, dx) in D4.iter() {
                let next_y: usize = y.wrapping_add(dy);
                let next_x: usize = x.wrapping_add(dx);
                if next_y < h && next_x < w {
                    if graph[next_y][next_x] == 'x' {
                        if !used[next_y][next_x].contains(&uf.root(node)) {
                            used[next_y][next_x].insert(uf.root(node));
                            *map_size.entry(uf.root(node)).or_insert(0) += 1;
                        }
                    }
                }
            }
        }
    }

    let mut res: usize = 1;
    for (_, &val) in map_size.iter() {
        res = res.max(val);
    }

    return res;
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