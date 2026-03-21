use proconio::input;
const INF: i64 = 1 << 60;
// ABC441F
// Q. Juedge whether the item = i should be taken or not
/* A. Reconstruction DP
DP の到達状態を後ろから復元し，最適解に含まれるかどうかを判定する解法
dp[i][j] 「1 番目から i 番目までの商品からいくつか（0 個でもよい）選び，値段の合計がちょうど j 円となるようにしたときの，価値の合計の最大値」
以下の情報を後ろから再構築する．
reachable[i][j] = 「1 番目から i 番目までの商品からいくつか選んで値段の合計を j 円としたとき，そこから先を適切に選ぶことで最終的な価値 X を達成できるかどうか」
use[i]：価値 X を達成するような最適解の中に，「商品 i を選ぶ」遷移が少なくとも 1 回は存在するかどうか
skip[i]：価値 X を達成するような最適解の中に，「商品 i を選ばない」遷移が少なくとも 1 回は存在するかどうか
*/
fn main() {
    input! {n: usize, m: usize, pv: [(usize, i64); n]}
    // 初期値をいじることによってちょうどj円になる場合のみを計算するDPを用いてdpテーブルを作る
    let dp: Vec<Vec<i64>> = run_dp(m, &pv);
    let reachable: Vec<Vec<bool>> = recon_reachable(m, &pv, &dp);
    let (may_use, can_skip): (Vec<bool>, Vec<bool>) = recon_use_skip(m, &pv, &dp, &reachable);
    let mut ans: Vec<char> = vec!['?'; n];
    for i in 0..n {
        if may_use[i] && !can_skip[i] {
            ans[i] = 'A';
        } else if may_use[i] && can_skip[i] {
            ans[i] = 'B';
        } else {
            ans[i] = 'C';
        }
    }
    println!("{}", ans.iter().collect::<String>());
}

fn run_dp(m: usize, pv: &[(usize, i64)]) -> Vec<Vec<i64>> {
    let n: usize = pv.len();
    let mut dp: Vec<Vec<i64>> = vec![vec![-INF; m + 1]; n + 1];
    // dp[i][j] := 1番目からi番目までの商品からいくつか選んで値段の合計をちょうどj円としたときの価値の合計の最大値
    // 初期値によってちょうどj円を実現する
    dp[0][0] = 0;
    for i in 0..n {
        for j in 0..=m {
            dp[i + 1][j] = dp[i][j];
            if j >= pv[i].0 && dp[i][j - pv[i].0] != -INF {
                dp[i + 1][j] = dp[i + 1][j].max(dp[i][j - pv[i].0] + pv[i].1);
            }
        }
    }
    dp
}

fn recon_reachable(m: usize, pv: &[(usize, i64)], dp: &Vec<Vec<i64>>) -> Vec<Vec<bool>> {
    let n: usize = pv.len();
    let mut reachable: Vec<Vec<bool>> = vec![vec![false; m + 1]; n + 1];
    let max_value: i64 = *dp[n].iter().max().unwrap();
    for j in 0..=m {
        if dp[n][j] == max_value {
            reachable[n][j] = true;
        }
    }
    for i in (0..n).rev() {
        let (price_item, value_item) = pv[i];
        for j in 0..=m {
            // skip the item
            if dp[i][j] == dp[i + 1][j] && reachable[i + 1][j] {
                reachable[i][j] = true;
            }
            // choose the item
            if j + price_item <= m
                && dp[i][j] + value_item == dp[i + 1][j + price_item]
                && reachable[i + 1][j + price_item]
            {
                reachable[i][j] = true;
            }
        }
    }
    reachable
}

fn recon_use_skip(
    m: usize,
    pv: &[(usize, i64)],
    dp: &Vec<Vec<i64>>,
    reachable: &Vec<Vec<bool>>,
) -> (Vec<bool>, Vec<bool>) {
    let n = pv.len();
    let mut may_use = vec![false; n];
    let mut can_skip = vec![false; n];

    // Iterate to find valid transitions
    for i in 0..n {
        let (p, v) = pv[i];
        for j in 0..=m {
            // We look at the DESTINATION state at i+1
            if !reachable[i + 1][j] {
                continue;
            }
            // Can we reach (i+1, j) by SKIPPING i?
            // This requires previous state (i, j) to match value
            if dp[i][j] == dp[i + 1][j] {
                // AND (i, j) must be valid/reachable itself?
                // Since you computed reachable separately, we assume it's correct.
                // But strictly, we only care if this transition exists on an optimal path.
                can_skip[i] = true;
            }
            // Can we reach (i+1, j) by USING i?
            // This requires previous state (i, j-p)
            if j >= p && dp[i][j - p] + v == dp[i + 1][j] {
                may_use[i] = true;
            }
        }
    }
    (may_use, can_skip)
}
