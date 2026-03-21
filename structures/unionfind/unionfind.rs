use petgraph::unionfind::UnionFind;

// dsu(disjoint set union)
fn main() {
    let mut uf = UnionFind::new(n);
    // root
    uf.find(x);
    // 同じ木に属するか？
    uf.equiv(x, y);
    // 木を合体する。
    uf.union(x, y);
    // 頂点のrootのリストをVecで返す。
    uf.into_labeling();
    // 参照するとき
    uf: &UnionFind<T>

    // tips
    // 根と頂点を比べることで、グループの数を調べられる。
    let mut group_num = 0;
    for x in 0..n {
        if x == uf.find(x) {
            group_num += 1;
        }
    }

    // unionfindのグループの大きさ
    let mut sizes: Vec<usize> = vec![1; n];
    // rootの大きさを変更する
    let sum_size: usize = sizes[uf.find(u)] + sizes[uf.find(v)];
    sizes[uf.find(u)] = sum_size;
    // judge all nodes connected
    let root: usize = uf.find(0);
    let mut is_connected = true;
    for v in 0..n {
        if uf.find(v) != root {
            is_connected = false;
        }
    }
}