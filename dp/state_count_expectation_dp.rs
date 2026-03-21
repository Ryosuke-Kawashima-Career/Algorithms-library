use proconio::input;
// educational DP J: Sushi
// count remain dishes!
// 残寿司数が同じ皿同士は区別する必要がないことに注意する!
fn main() {
    // a: 寿司の数(1..=3)
    input!{n: usize, a: [usize; n]}
    // 操作回数の期待値 = dp[i: 残り1][j: 残り2][k: 残り3] (-1: not updated)
    let mut dp: Vec<Vec<Vec<f64>>> = vec![vec![vec![-1.0; n+1]; n+1]; n+1];
    // 寿司の皿の数を数える
    let mut count: [usize; 4] = [0; 4];
    for i in 0..n {
        count[a[i]] += 1;
    }
    let expect = memo_rec(count, &mut dp);
    println!("{}", expect);
}

// 漸化式から立式する(方程式で移項を用いる)
// dp[i][j][k] = (n-i-j-k)/n * dp[i][j][k] + i/n * dp[i-1][j][k] + j/n * dp[i+1][j-1][k] + k/n * dp[i][j+1][k-1] + 1
// dp[i][j][k] = n/(i+j+k) * (i/n * dp[i-1][j][k] + j/n * dp[i+1][j-1][k] + k/n * dp[i][j+1][k-1] + 1)
// dp[i][j][k] = 1/(i+j+k) * (i * dp[i-1][j][k] + j * dp[i+1][j-1][k] + k * dp[i][j+1][k-1] + n)
fn memo_rec(count: [usize; 4], dp: &mut Vec<Vec<Vec<f64>>>) -> f64 {
    let (one, two, three) = (count[1], count[2], count[3]);
    let n: usize = dp.len() - 1;
    // if visited return
    if dp[one][two][three] != -1.0 {
        return dp[one][two][three];
    }
    // base case
    if one == 0 && two == 0 && three == 0 {
        return 0.0;
    }
    // 今回の操作分の1
    let mut res = 1.0;

    for remain in 1..=3 {
        if count[remain] >= 1 {
            let mut count_new = count;
            count_new[remain-1] += 1;
            count_new[remain] -= 1;
            res += memo_rec(count_new, dp) * (count[remain] as f64 / n as f64);
        }
    }
    res *= n as f64 / (one + two + three) as f64;

    // memo and return
    dp[one][two][three] = res;
    return res;
}