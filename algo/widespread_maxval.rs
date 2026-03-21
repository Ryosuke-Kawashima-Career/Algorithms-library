use proconio::input;
// JOI2012本選4-釘
// 最大値の伝搬(正三角形なのでうまくいっている)
// 根から葉へ水が減っていきながら伝わるイメージ
fn main() {
    input!{n: usize, m: usize}
    let mut roots: Vec<Vec<i64>> = vec![vec![0; n+1]; n+1];
    // (a, b), (a+x, b), (a+x, b+x): 三角形の頂点
    for _ in 0..m {
        input!{a: usize, b: usize, x: i64}
        let tree_size: i64 = x + 1;
        roots[a][b] = roots[a][b].max(tree_size);
    }

    // 最大値の伝搬
    for i in 1..=n {
        for j in 1..=n {
            // -1: 水の勢いが減少するイメージ
            roots[i][j] = roots[i][j].max(roots[i-1][j-1]-1).max(roots[i-1][j]-1);
        }
    }

    let mut ans: usize = 0;
    for i in 0..=n {
        for j in 0..=n {
            if roots[i][j] > 0 {
                ans += 1;
            }
        }
    }
    println!("{}", ans);
}