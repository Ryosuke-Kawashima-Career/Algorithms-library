use proconio::input;
const INF: i64 = 1 << 60;
// abc076D
// 1. 場合分けなどで何らかの最善戦略を探す
//  最善となる戦略を先手・後手ともに見つける
//  後手は「先手がどう動くか」が分からないと方針が立たないことが多いので、先手の行動を場合分けしてみる
// 2. 最大化側はx以上・最小化側はx以下を達成できるか考える(lower upper bound)
//  最善戦略を探す時や、その戦略かを確かめる時は、以下を考える
//  最大化したい側は、最適に行動すれば少なくとも x 以上になる
//  最小化したい側は、最適に行動すれば多くても x 以下になる
//  すると、両方が最適に行動した際に x になることが分かる
// 「仮に先手がこのように行動すると、後手は必ず x 以下にできる」というように場合分けした後で使えることもある
fn main() {
    // Q. 得点の差の絶対値を，先手: 最大化 後手: 最小化　(1枚は必ず引く，最後のカードを持つ)
    // A. 先手の行動を場合分けをして、得点の上限や下限が抑えられるかを考える(メモ化再帰)
    input!{n: usize, z: i64, w: i64, a: [i64; n]}
    // max abs diff = dp[last index][first or second]
    let mut dp: Vec<Vec<i64>> = vec![vec![-1; 2]; n];

    println!("{}", min_max(0, 0, z, w, &a, &mut dp));
}

// x: 先手が現在持っているカード y: 後手が現在持っているカード
fn min_max(id: usize, turn: usize, x: i64, y: i64, 
    cards: &Vec<i64>, dp: &mut Vec<Vec<i64>>
) -> i64 {
    let n: usize = cards.len();
    // base case, end case
    if id == n {
        return (x - y).abs();
    }
    // if visited return
    if dp[id][turn] != -1 {
        return dp[id][turn];
    }

    if turn == 0 {
        let mut max_score: i64 = 0;
        // i: 最後に引かれたカード
        for i in id..n {
            let cur_score: i64 = min_max(i+1, turn^1, cards[i], y, cards, dp);
            max_score = max_score.max(cur_score);
        }
        dp[id][turn] = max_score;
    } else {
        let mut min_score: i64 = INF;
        for i in id..n {
            let cur_score: i64 = min_max(i+1, turn^1, x, cards[i], cards, dp);
            min_score = min_score.min(cur_score);
        }
        dp[id][turn] = min_score;
    }

    // memo and return
    return dp[id][turn];
}