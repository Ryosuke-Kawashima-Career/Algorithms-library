use proconio::input;
const INF: usize = 1 << 60;
// ABC419E
// Q. Get the minimum number of operations turing the sums of all the subsequences == 0 (mod m)
// A. DP: the previous state determines the current one
// 同じ周期の位相にいる数値を同じ数にする。Ai,AL+i,A2L+i,… をMで割った余りがすべてjにする(Lが周期)
fn main() {
    // m: mod, l: length
    input!{n: usize, m: usize, l: usize, a: [usize; n]}
    // dp[i][j] = min operation of A0 + .. + Ai == j % m
    let mut dp: Vec<Vec<usize>> = vec![vec![INF; m]; n+1];
    dp[0][0] = 0;
    for start in 1..=l {
        for mod_Ai in 0..m {
            let mut cost: usize = 0;
            let mut start_next_period: usize = start-1;
            while start_next_period < n {
                if mod_Ai >= a[start_next_period] % m {
                    cost += mod_Ai - a[start_next_period] % m;
                } else {
                    cost += mod_Ai + m - a[start_next_period] % m;
                }
                // the periodicity is l.
                start_next_period += l;
            }
            for mod_sum in 0..m {
                if dp[start][(mod_sum + mod_Ai) % m] > dp[start-1][mod_sum] + cost {
                    dp[start][(mod_sum + mod_Ai) % m] = dp[start-1][mod_sum] + cost;
                }
            }
        }
    }
    println!("{}", dp[l][0]);
}