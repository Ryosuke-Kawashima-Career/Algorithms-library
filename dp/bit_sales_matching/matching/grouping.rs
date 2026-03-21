use proconio::{input, marker::Usize1};
// abc310D
// t: チーム数, ab: 嫌いな人同士
// 嫌いな人同士が違うチームになるように分ける
// チームごとにメンバーをまとめて決定する解法: O(3^N*T)
fn main() {
    input!{n: usize, t: usize, m: usize, ab: [(Usize1, Usize1); m]}
    // i番目とj番目の仲が悪い = hate[i]のjbit目が1
    let mut hate: Vec<usize> = vec![0; n];
    for &(a, b) in ab.iter() {
        hate[a] |= 1 << b;
        hate[b] |= 1 << a;
    }
    let bits: usize = 1 << n;

    // possible_teams[S] := S からなるチームを作ったとき、相性の悪い選手の組がいない
    // O(2^N N^2/w) 時間
    let mut possible_teams: Vec<bool> = vec![false; bits];
    // bitはグループにいる人
    for bit in 0..bits {
        let mut hates: usize = 0;
        for i in 0..n {
            if bit >> i & 1 == 1 {
                hates |= hate[i];
            }
        }
        if bit & hates == 0 {
            possible_teams[bit] = true;
        }
    }

    // dp[S][t]:=(Sに含まれる選手を、互いに相性の悪い選手を含まないtチームに分ける場合の数)
    let mut dp: Vec<Vec<usize>> = vec![vec![0; t+1]; bits];
    dp[0][0] = 1;
    for bit in 0..bits {
        // 残っているうち番号の一番小さい選手が属するチームを全探索
        let c = bit | (bit + 1);
        let mut j = c;
        while j < bits {
            if possible_teams[j ^ bit] {
                for k in 0..t {
                    dp[j][k+1] += dp[bit][k];
                }
            }
            j += 1;
            j |= c;
        }
    }
    println!("{}", dp[bits-1][t]);
}