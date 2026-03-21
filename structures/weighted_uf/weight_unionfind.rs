use proconio::input;
// abc087d
// 座標の相対位置の問題に使える(与えられた条件から座標を特定できるか？)
// xRi-xLi=Diをすべて満たす数列(x1..xn)があるか?
// 軸が複数あるときは軸ごとに計算するabc320d
fn main() {
    input!{n: usize, m: usize}
    let mut wuf = WeightedUnionFind::new(n);
    let mut is_ok: bool = true;
    for _ in 0..m {
        input!{l: usize, r: usize, d: i64}
        let (l, r) = (l-1, r-1);
        // (u, v)の順序に意味があることに注意!: l - r = d
        if !wuf.union(l, r, d) {
            is_ok = false;
        }
    }

    if is_ok {
        println!("Yes");
    } else {
        println!("No");
    }
}

// 重み付きUnion-Find
// 通常のUnion-Findとの違いは以下
// union()が二点間のエネルギー差 e を受け取る。例えばunion(i, j, e)は、jを基準点としたときのiのポテンシャル
// 任意の二点間のエネルギー差を求めるpotential()のサポート。例えばpotential(i, j)はjを基準としたときのiのポテンシャル
#[derive(Debug, Clone, PartialOrd, Ord, PartialEq, Eq, Hash)]
struct WeightedUnionFind {
    // 親ノード
    parent: Vec<usize>,
    // 木の高さ(パフォーマンス用)
    rank: Vec<usize>,
    // それぞれの連結成分のノード数
    size_map: Vec<usize>,

    // ポテンシャル
    // potential[i]は、parent[i]を基準点としたときのiのポテンシャルエネルギー
    potential: Vec<i64>,
}
impl WeightedUnionFind {
    fn new(n: usize) -> Self {
        Self {
            parent: (0..n).collect(),
            rank: vec![0; n],
            size_map: vec![1; n],
            potential: vec![0; n],
        }
    }

    // ノードiが属する木のルートノードを返す
    fn root(&mut self, i: usize) -> usize {
        if self.parent[i] == i {
            return i;
        }
        let old_parent = self.parent[i];
        let new_parent = self.root(self.parent[i]);
        if new_parent != old_parent {
            self.parent[i] = new_parent;
            self.potential[i] += self.potential[old_parent];
        }
        self.parent[i]
    }

    #[allow(dead_code)]
    fn potential(&mut self, x: usize) -> i64 {
        self.root(x);
        return self.potential[x];
    }

    // ノードiが属する連結成分のノード数を返す
    #[allow(dead_code)]
    fn size(&mut self, i: usize) -> usize {
        let root = self.root(i);
        self.size_map[root]
    }

    // jを基準点としたときのiのポテンシャルエネルギーを求める
    // i, jが同じ連結成分に属していることが前提であるため、基準点をself.root(i)に取り直して良い:
    // U(i) - U(j) = (U(i) - U(root)) - (U(j) - U(root))
    #[allow(dead_code)]
    fn diff(&mut self, i: usize, j: usize) -> i64 {
        assert!(self.equiv(i, j));
        self.root(i);
        self.root(j);
        self.potential[i] - self.potential[j]
    }

    // それぞれの連結成分に含まれるノードの一覧を返す
    // キーはルートノード
    #[allow(dead_code)]
    fn list_groups(&mut self) -> std::collections::HashMap<usize, Vec<usize>> {
        let mut ret = std::collections::HashMap::new();
        for i in 0..self.parent.len() {
            ret.entry(self.root(i)).or_insert(vec![]).push(i);
        }
        ret
    }

    #[allow(dead_code)]
    fn groups(&mut self) -> usize {
        let mut res = 0;
        for i in 0..self.parent.len() {
            if self.root(i) == i {
                res += 1;
            }
        }
        res
    }

    //ノードi, jが同じ木に属するか判定する
    #[allow(dead_code)]
    fn equiv(&mut self, i: usize, j: usize) -> bool {
        self.root(i) == self.root(j)
    }

    // ノードiが属する木とノードjが属する木を統合する
    //
    // eはjを基準点としたときのiのポテンシャルエネルギー
    // メソッド内の最初にself.root(i), self.root(j)を呼んだ時点で、次のように、iとi_rootが直接繋がっており、jとj_rootも直接繋がっている木構造になっている
    // ただし、potential[i] == x, potential[j] == yとなるようにx, yを定義した(なお、i == i_rootなどの場合もx == 0などとすれば同じ図で良い)
    //
    //  i_root o      o j_root
    //         ↑      ↑
    //       x |      | y
    //         |      |
    //       i o      o j
    //
    // このとき、i_rootとj_rootを接続したい
    //
    //             z
    //  i_root o ---→ o j_root
    //         ↑      ↑
    //       x |      | y
    //         |      |
    //       i o      o j
    //
    // ただし、e = U(i) - U(j) = (z + x) - yを満たしたいため、z = e + y - xが得られる
    // つまり、potential[i_root] = e + potential[j] - potential[i]とすれば良い
    #[allow(dead_code)]
    fn union(&mut self, i: usize, j: usize, e: i64) -> bool {
        let i_root = self.root(i);
        let j_root = self.root(j);
        if i_root == j_root {
            return self.diff(i, j) == e;
        }
        if self.rank[i_root] < self.rank[j_root] {
            self.size_map[j_root] += self.size_map[i_root];
            self.parent[i_root] = j_root;
            self.potential[i_root] = self.potential[j] + e - self.potential[i];
        } else {
            self.size_map[i_root] += self.size_map[j_root];
            self.parent[j_root] = i_root;
            self.potential[j_root] = self.potential[i] - e - self.potential[j];
            if self.rank[i_root] == self.rank[j_root] {
                self.rank[i_root] += 1;
            }
        }
        true
    }
}