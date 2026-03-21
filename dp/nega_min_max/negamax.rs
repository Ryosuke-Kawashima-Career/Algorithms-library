use proconio::input;
const INF: i64 = 1 << 60;
// 区間DP(EducationalDP L)
// 先頭と末尾の値しか使えない。
// ゼロサムゲーム -> ミニマックス法、ネガマックス法

// 先手は最も大きな評価値の手を選び、後手は最も小さな評価値の手を選ぶ。
// ここで後手番のときに評価値の符号を反転すると、先手と同様に後手でも最大な評価値の手を選べばよい。
// つまり、手番を変えて思考ルーチンを呼び出すときは、その返り値 (評価値) にマイナス符号をつけて符号を反転。
// この方法を「ネガマックス法 (nega-max method)」
fn main() {
    input!{n: usize, a: [i64; n]}
    let mut dp: Vec<Vec<i64>> = vec![vec![-INF; n+1]; n+1];
    let ans: i64 = nega_max(0, n, &mut dp, &a);

    println!("{}", ans);
}

// [l r)の範囲で計算する。First: max(x-y), Second: max(y-x)
fn nega_max(l: usize, r: usize, dp: &mut Vec<Vec<i64>>, a: &Vec<i64>) -> i64 {
    let mut res: i64 = -INF;
    // visited
    if dp[l][r] != -INF {
        res = dp[l][r];
        return res;
    }
    // base case
    if l >= r {
        return 0;
    }

    // negamax(-でx-yの符号を反転させる)
    let score_l: i64 = -nega_max(l+1, r, dp, a) + a[l];
    let score_r: i64 = -nega_max(l, r-1, dp, a) + a[r-1];
    res = res.max(score_l).max(score_r);

    // memo and return
    dp[l][r] = res;
    return res;
}