use proconio::input;
// bitDP
// 鉄則A23
fn main() {
    input!{n: usize, m: usize, a: [[usize; n]; m]}
    // dp[i][bit] i: 1indexed
    let mut sets: usize = 1 << n;
    let mut dp: Vec<Vec<usize>> = vec![vec![m+1; sets]; m+1];
    dp[0][0] = 0;

    // 配る遷移の方がわかりやすい。
    for i in 0..m {
        for bit in 0..sets {
            dp[i+1][bit] = dp[i+1][bit].min(dp[i][bit]);

            // bitで使用可能かを表現する。
            let mut availables: usize = 0;
            for j in 0..n {
                availables += a[i][j] << j;
            }
            dp[i+1][bit | availables] = dp[i+1][bit | availables]
            .min(dp[i][bit] + 1);
        }
    }

    if dp[m][sets-1] == m+1 {
        println!("-1");
    } else {
        println!("{}", dp[m][sets-1]);
    }
}

// もらう遷移の場合
const INF: usize = 1 << 60;

fn main() {
    input!{n: usize, m: usize, a: [[usize; n]; m]}
    let bits: usize = 1 << n;
    // 最小何枚のクーポンを使うか = dp[i: クーポン][j: bit(bought goods)]
    let mut dp: Vec<Vec<usize>> = vec![vec![INF; bits]; m+1];
    dp[0][0] = 0;
    for i in 1..=m {
        let mut goods: usize = 0;
        for j in 0..n {
            if a[i-1][j] == 1 {
                // goods += 1 << j; is ok, too.
                goods |= 1 << j;
            }
        }

        for bit in 0..bits {
            // クーポンを使わないとき
            dp[i][bit] = dp[i][bit].min(dp[i-1][bit]);

            dp[i][bit|goods] = dp[i][bit|goods].min(dp[i-1][bit]+1);
        }
    }

    let mut ans: usize = INF;
    for i in 0..=m {
        ans = ans.min(dp[i][bits-1]);
    }
    if ans == INF {
        println!("-1");
    } else {
        println!("{}", ans);
    }
}