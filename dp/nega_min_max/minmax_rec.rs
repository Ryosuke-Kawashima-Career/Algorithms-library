use proconio::input;
const INF: i64 = 1 << 60;
// 典型A35
// min-max法: DP(先手と後手で戦略が違う)
// 先手はスコアを最大化し、後手はスコアを最小化する
fn main() {
    input!{n: usize, a: [i64; n]}
    // score = dp[i: phase][j: game]
    let mut dp: Vec<Vec<i64>> = vec![vec![0; n]; n];
    let score: i64 = minmax(0, 0, &a, &mut dp);

    println!("{}", score);
}

fn minmax(turn: usize, game: usize, a: &Vec<i64>, dp: &mut Vec<Vec<i64>>) -> i64 {
    let n: usize = dp.len();
    // base case
    if turn == n-1 {
        dp[turn][game] = a[game];
        return dp[turn][game];
    }
    // if updated, returns value
    if dp[turn][game] != 0 {
        return dp[turn][game];
    }

    let score_l: i64 = minmax(turn+1, game, a, dp);
    let score_r: i64 = minmax(turn+1, game+1, a, dp);
    
    // 帰りがけ: 子供の情報を親に
    let score: i64 = if turn % 2 == 0 {
        score_l.max(score_r)
    } else {
        score_l.min(score_r)
    };

    // メモ化
    dp[turn][game] = score;
    return score;
}