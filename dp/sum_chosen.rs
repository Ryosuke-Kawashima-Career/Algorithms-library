use proconio::input;
// arc060C
// N=50: O(N^4)なら間に合う
// Q. カードの中から1枚以上を選び、 選んだカードに書かれた整数の平均をちょうどAにする場合の数
fn main() {
    input!{n: usize, a: usize, x: [usize; n]}
    let total: usize = x.iter().sum();
    // dp[checked item(0..i)][choice][sum] = cases
    let mut dp: Vec<Vec<Vec<usize>>> = vec![vec![vec![0; total+1]; n+1]; n+1];
    dp[0][0][0] = 1;
    for i in 1..=n {
        for j in 0..=i {
            for k in 0..=total {
                dp[i][j][k] += dp[i-1][j][k];
    
                if j >= 1 && k >= x[i-1] {
                    dp[i][j][k] += dp[i-1][j-1][k-x[i-1]];
                }
            }
        }
    }

    let mut ans: usize = 0;
    for j in 1..=n {
        if a * j <= total {
            ans += dp[n][j][a*j];
        }
    }
    println!("{}", ans);
}