use proconio::input;
// JOI2012本選4-釘
// 累積和の正三角形版
// O(膜の数*面積) -> O(膜の数+面積)に改善する
fn main() {
    input!{n: usize, m: usize}
    let mut imos: Vec<Vec<i64>> = vec![vec![0; n+3]; n+3];
    // (a, b), (a+x, b), (a+x, b+x): 長方形の累積和の下半分
    // 三角形の問題を四角形の問題にいいかえる
    // 累積和の取り方を斜めに足し算することから逆算!して考える
    for _ in 0..m {
        input!{a: usize, b: usize, x: usize}
        // 1. 適切な場所に数字を足す(数字が打ち消しあうようにする)
        imos[a][b] += 1;
        imos[a][b+1] -= 1;
        imos[a+x+1][b] -= 1;
        imos[a+x+1][b+x+2] += 1;
        imos[a+x+2][b+1] += 1;
        imos[a+x+2][b+x+2] -= 1;
    }
    // 2. 左から右に累積和を取る
    for i in 1..n+3 {
        for j in 1..n+3 {
            imos[i][j] += imos[i][j-1];
        }
    }
    // 3. 上から下に累積和を取る
    for i in 1..n+3 {
        for j in 1..n+3 {
            imos[i][j] += imos[i-1][j];
        }
    }
    // 4. 左上から右下に累積和を取る
    for i in 1..n+3 {
        for j in 1..n+3 {
            imos[i][j] += imos[i-1][j-1];
        }
    }

    let mut ans: usize = 0;
    for i in 0..n+3 {
        for j in 0..n+3 {
            if imos[i][j] > 0 {
                ans += 1;
            }
        }
    }
    println!("{}", ans);
}