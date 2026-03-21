use proconio::{input, marker::Usize1};
// ABC436E
// Q. You are given a vector using 1..=n. What operations are required first to coduct minimum number of operations to sort the vector?
// A. (1,2,…,N) の並べ替えである列 P に対して、頂点 1, 頂点 2,…, 頂点 N の N 個の頂点からなるグラフを考え、i=1,2,…,N に対して頂点 i と頂点 P i を結ぶ辺を追加します。 このグラフの連結成分の個数を C としたとき、K=N−C
// A. （操作をする時点での P に対応する）グラフで同じ連結成分にある 2 頂点を選んで操作を行うことを K 回行う必要があることがわかります。 逆に、同じ連結成分にある 2 頂点であれば、どの組を選んでもよい
/*: Proof
Decompose the permutation into disjoint cycles; let the cycle lengths be c1, c2, ... .
For a cycle of length c, the number of unordered pairs inside that cycle is C(c,2) = c(c−1)/2.
Summing over cycles gives total valid pairs: sum c(c−1)/2.
sum c(c−1)/2 = (sum c^2 − sum c) / 2.
sum c = n (every element belongs to exactly one cycle).
In the code, cycle_sizes stores for each element the length of its cycle, so
sum of entries = sum over cycles of c·c = sum c^2.
Hence (sum c^2 − n) / 2 = (cycle_sizes.iter().sum::<usize>() − n) / 2.
 */
fn main() {
    input! {n: usize, p: [Usize1; n]}
    let mut cycle_sizes: Vec<usize> = vec![0; n];
    // (0..n): Objective
    for v in 0..n {
        if cycle_sizes[v] == 0 {
            let mut p_vertex = v;
            let mut cycle_size = 0;
            // Calculate the cycle size: 連結成分の大きさ
            loop {
                cycle_size += 1;
                p_vertex = p[p_vertex];
                if p_vertex == v {
                    break;
                }
            }
            // Update the cycle sizes: 連結成分の大きさを更新する
            loop {
                cycle_sizes[p_vertex] = cycle_size;
                p_vertex = p[p_vertex];
                if p_vertex == v {
                    break;
                }
            }
        }
    }
    let ans = (cycle_sizes.iter().sum::<usize>() - n) / 2;
    println!("{}", ans);
}
