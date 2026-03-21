use proconio::{input, marker::Chars};
const MOD: usize = 10_007;
// JOI 2014 予選 4 - 部活のスケジュール表
// dp[cur item][state by bit]の遷移を計算する
// prev_bit & bit != 0 の時にノードを共有している
fn main() {
    input!{n: usize, joi: Chars}
    let mut schedules: Vec<usize> = vec![3; n];
    for i in 0..n {
        match joi[i] {
            'J' => schedules[i] = 0,
            'O' => schedules[i] = 1,
            'I' => schedules[i] = 2,
            _ => println!("Input Error!"),
        }
    }
    // J, O, I
    let bits: usize = 1 << 3;
    // cases = dp[i: day][j: state of who is in charge]
    let mut dp: Vec<Vec<usize>> =  vec![vec![0; bits]; n+1];
    // J has the key, at first
    dp[0][1] = 1;
    for day in 1..=n {
        for prev_bit in 0..bits {
            for bit in 0..bits {
                // 責任者が出席, 前日に出席した人が当日にも出席するか？
                if bit >> schedules[day-1] & 1 == 1 && prev_bit & bit != 0 {
                    dp[day][bit] += dp[day-1][prev_bit];
                    dp[day][bit] %= MOD;
                }
            }
        }
    }

    let mut ans: usize = 0;
    for bit in 0..bits {
        ans += dp[n][bit];
        ans %= MOD;
    }
    println!("{}", ans);
}