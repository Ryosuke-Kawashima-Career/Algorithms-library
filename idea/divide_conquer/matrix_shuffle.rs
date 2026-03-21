use proconio::input;
const MOD: usize = 998244353;
// arc107C: 操作が独立
// 和がK以下の行同士，列同士でswap -> 何通りの行列?
// 列swapをしても、行swapの条件に一切影響がない
// 「行swapしか出来ない場合の答え」 × 「列swapしか出来ない場合の答え」
fn main() {
    // a[i][j] = 1..=n*n
    input!{n: usize, k: usize, a: [[usize; n]; n]}
    let rows = a.clone();
    let mut cols: Vec<Vec<usize>> = Vec::new();
    for j in 0..n {
        let mut col: Vec<usize> = Vec::new();
        for i in 0..n {
            col.push(a[i][j]);
        }
        cols.push(col);
    }

    let swap_row = cout_swap(&rows, k);
    let swap_col = cout_swap(&cols, k);
    let ans = swap_row * swap_col;
    println!("{}", ans % MOD);
}

fn cout_swap(vectors: &Vec<Vec<usize>>, k: usize) -> usize {
    let n: usize = vectors.len();
    let mut uf = UnionFind::new(n);

    for i1 in 0..n {
        for i2 in i1+1..n {
            let mut all_ok = true;
            for j in 0..n {
                if vectors[i1][j] + vectors[i2][j] > k {
                    all_ok = false;
                }
            }
            if all_ok {
                uf.union(i1, i2);
            }
        }
    }

    let mut cluster_sizes: Vec<usize> = Vec::new();
    for v in 0..n {
        if uf.root(v) == v {
            cluster_sizes.push(uf.size(v));
        }
    }

    let mut res: usize = 1;
    for &size in cluster_sizes.iter() {
        res *= factorial(size);
        res %= MOD;
    }

    return res % MOD;
}

fn factorial(num: usize) -> usize {
    if num == 0 {
        return 1;
    }

    return (num * (factorial(num-1) % MOD)) % MOD;
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