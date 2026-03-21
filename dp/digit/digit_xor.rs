use proconio::input;
const DIGIT: usize = 60;
// abc347D
// bitの桁ごとに計算する
fn main() {
    // a = popcount(x), b = popcount(y), c = x ^ y
    input!{a: usize, b: usize, c: usize}
    // dp[i][j][k] = (下からi-bit 決めていて、popcount(X)=j,popcount(Y)=kのときの(X,Y))
    let dp: Vec<Vec<Vec<usize>>> = run_dp(c);
    if dp[DIGIT-1][a][b] == 0 {
        println!("-1");
        return;
    }
    let (x, y) = recon_dp(a, b, c, &dp);
    println!("{} {}", x, y);
}

// 上の桁から見る
fn recon_dp(mut a: usize, mut b: usize, c: usize, dp: &Vec<Vec<Vec<usize>>>) -> (usize, usize) {
    let mut x: usize = 0;
    let mut y: usize = 0;
    for i in (1..DIGIT).rev() {
        let digit = i;
        // a = popcount(x), b = popcount(y)
        if c >> digit & 1 == 1 {
            // 1 ^ 0 == 1
            if a >= 1  && dp[i-1][a-1][b] > 0 {
                x |= 1 << digit;
                a -= 1;
            } else if b >= 1 && dp[i-1][a][b-1] > 0 {
                y |= 1 << digit;
                b -= 1;
            }
        } else {
            // 0 ^ 0 ができないとき
            if a >= 1 && b >= 1 && dp[i-1][a][b] == 0 {
                x |= 1 << digit;
                y |= 1 << digit;
                a -= 1;
                b -= 1;
            }
        }
    }
    // corner case
    if a == 1 {
        x |= 1;
    }
    if b == 1 {
        y |= 1;
    }
    return (x, y);
}

// 下の桁から見る
fn run_dp(c: usize) -> Vec<Vec<Vec<usize>>> {
    // dp[i][j][k] = (下からi-bit 決めていて、popcount(X)=j,popcount(Y)=kのときの(X,Y)の個数)
    let mut dp: Vec<Vec<Vec<usize>>> = vec![vec![vec![0; DIGIT+1]; DIGIT+1]; DIGIT];
    // base case: c >> 0 & 1
    if c & 1 == 1 {
        dp[0][1][0] = 1;
        dp[0][0][1] = 1;
    } else {
        dp[0][0][0] = 1;
        dp[0][1][1] = 1;
    }
    for i in 1..DIGIT {
        let digit = i;
        for j in 0..=DIGIT {
            for k in 0..=DIGIT {
                if c >> digit & 1 == 1 {
                    // 1 ^ 0 == 0 ^ 1 == 1
                    if j >= 1 {
                        dp[i][j][k] += dp[i-1][j-1][k];
                    }
                    if k >= 1 {
                        dp[i][j][k] += dp[i-1][j][k-1];
                    }
                } else {
                    // 0 ^ 0 == 1 ^ 1 == 0
                    dp[i][j][k] += dp[i-1][j][k];
                    if j >= 1 && k >= 1 {
                        dp[i][j][k] += dp[i-1][j-1][k-1];
                    }
                }
            }
        }
    }
    return dp;
}