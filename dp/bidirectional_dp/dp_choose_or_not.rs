use proconio::input;
const INF: i64 = 1 << 60;
// ABC441F
// Q. Juedge whether the item = i should be taken or not
/* A. Bidirectional DP
1. 商品 i を除いた N−1 個の商品から 、値段の合計が M 円以下かつ価値の合計が X となるように商品を選ぶことができるか？
2. 商品 i を除いた N−1 個の商品から 、値段の合計が M−Pi円以下かつ価値の合計が X−Viとなるように商品を選ぶことができるか？
分類 A: 1. の条件をみたさないが、2. の条件をみたす。
分類 B: 1. の条件をみたし、2. の条件もみたす。
分類 C: 1. の条件をみたすが、2. の条件をみたさない。
*/
fn main() {
    input! {n: usize, m: usize, pv: [(usize, i64); n]}
    // Forward DP
    let dp_left = run_forward_dp(m, &pv);
    // Backward DP
    let dp_right = run_backward_dp(m, &pv);

    // Judge whether the items are necessary or not
    let mut max_value = dp_left[n][m];
    let mut ans = vec!['?'; n];
    for i in 0..n {
        let (price_item, value) = pv[i];
        let mut max_without_item = 0;
        for price_left in 0..=m {
            max_without_item =
                max_without_item.max(dp_left[i][price_left] + dp_right[i + 1][m - price_left]);
        }
        let mut max_with_item = 0;
        for price_left in 0..=m {
            if m >= price_item + price_left {
                max_with_item = max_with_item
                    .max(dp_left[i][price_left] + dp_right[i + 1][m - price_item - price_left]);
            }
        }
        if max_without_item < max_value {
            ans[i] = 'A';
        } else if max_with_item + value < max_value {
            ans[i] = 'C';
        } else {
            ans[i] = 'B';
        }
    }
    println!("{}", ans.iter().collect::<String>());
}

fn run_forward_dp(m: usize, pv: &Vec<(usize, i64)>) -> Vec<Vec<i64>> {
    let n = pv.len();
    let mut dp = vec![vec![-INF; m + 1]; n + 1];
    for price in 0..=m {
        dp[0][price] = 0;
    }
    for i in 0..n {
        let (price_item, value) = pv[i];
        for price in 0..=m {
            dp[i + 1][price] = dp[i][price];
            if price >= price_item {
                dp[i + 1][price] = dp[i + 1][price].max(dp[i][price - price_item] + value);
            }
        }
    }
    dp
}

fn run_backward_dp(m: usize, pv: &Vec<(usize, i64)>) -> Vec<Vec<i64>> {
    let n = pv.len();
    let mut dp = vec![vec![-INF; m + 1]; n + 1];
    for price in 0..=m {
        dp[n][price] = 0;
    }
    for i in (0..n).rev() {
        let (price_item, value) = pv[i];
        for price in 0..=m {
            dp[i][price] = dp[i + 1][price];
            if price >= price_item {
                dp[i][price] = dp[i][price].max(dp[i + 1][price - price_item] + value);
            }
        }
    }
    dp
}
