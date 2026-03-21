use proconio::input;
use petgraph::unionfind::UnionFind;
// 典型68
// unionfindで計算できるかを考える
// クエリ先読み
fn main() {
    input!{n: usize, q: usize, queries: [(usize, usize, usize, i64); q]}
    let mut uf = UnionFind::new(n);
    // sums[i] = a[i] + a[i+1]
    let mut sums: Vec<i64> = vec![0; n-1];
    let mut a: Vec<i64> = vec![0; n];
    // クエリの先読み
    for &(query, x, _, v) in queries.iter() {
        let x = x-1;
        if query == 0 {
            sums[x] = v;
        }
    }
    for i in 0..n-1 {
        a[i+1] = sums[i] - a[i];
    }

    for &(query, x, y, v) in queries.iter() {
        let (x, y) = (x-1, y-1);

        if query == 0 {
            uf.union(x, y);
        } else {
            if !uf.equiv(x, y) {
                println!("Ambiguous");
                continue;
            }
            // a[x](=v) -- sums[x] --- a[y]
            // 偶奇で場合分けする(部分的だが符号の正負が反転する)
            let ay: i64 = if (x + y) % 2 == 0 {
                a[y] + (v - a[x])
            } else {
                a[y] - (v - a[x])
            };
            println!("{}", ay);
        }
    }
}