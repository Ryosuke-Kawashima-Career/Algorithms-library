use proconio::input;
// 鉄則A57
// ダブリング
fn main() {
    input! {n: usize, q: usize, a: [usize; n]}
    // 2^i日後にjからdp[i][j]に行く
    let mut dp: Vec<Vec<usize>> = vec![vec![0; n + 1]; 32];
    for j in 1..=n {
        dp[0][j] = a[j - 1];
    }
    for i in 1..32 {
        for j in 1..=n {
            // 2^i = 2^(i-1) * 2^(i-1)
            dp[i][j] = dp[i - 1][dp[i - 1][j]];
        }
    }

    for _ in 0..q {
        // x: 現在地, y: 操作回数
        input! {mut x: usize, y: usize}
        for i in (0..32).rev() {
            // yを二進法に直す。大きな桁から計算する。
            if y >> i & 1 == 1 {
                x = dp[i][x];
            }
        }
        println!("{}", x);
    }
}
