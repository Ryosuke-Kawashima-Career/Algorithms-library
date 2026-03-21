use proconio::input;
const INF: i64 = 1_000_000_007;
// abc374e
// 「最小値を最大化」(または「最大値を最小化」) する問題
// 答えとなる値を仮に決め打って、その値が達成可能かを判定しながら二分探索する (いわゆる 決め打ち二分探索 ) テクニックが有効
// Q. 予算をX円までかけることができるとき、製造能力をw以上にできるか?
// Q. s: a個生産しp円のコスト vs t: b個生産しq円のコスト
// A. 条件: 機械SiがBi台以下しか導入されない。機械TiがAi台以下しか導入されない。-> 全探索!!!
fn main() {
    input!{n: usize, x: i64, apbq: [(i64, i64, i64, i64); n]}
    // binary search
    let mut wl: i64 = 0;
    let mut wr: i64 = INF;
    while wr - wl > 1 {
        let w_mid: i64 = (wl + wr) / 2;
        if budget(w_mid, &apbq) <= x {
            wl = w_mid;
        } else {
            wr = w_mid;
        }
    }
    println!("{}", wl);
}

// w: amount of production
fn budget(w: i64, apbq: &Vec<(i64, i64, i64, i64)>) -> i64 {
    let mut min_budget: i64 = 0;
    for &(amount1, price1, amount2, price2) in apbq.iter() {
        let mut min_cost: i64 = INF;
        for i1 in 0..=amount2 {
            let produced1: i64 = amount1 * i1;
            let produced2: i64 = 0.max(w - produced1);
            let i2: i64 = div_ceil(produced2, amount2);
            let cost: i64 = price1 * i1 + price2 * i2;
            min_cost = min_cost.min(cost);
        }
        for i2 in 0..=amount1 {
            let produced2: i64 = amount2 * i2;
            let produced1: i64 = 0.max(w - produced2);
            let i1: i64 = div_ceil(produced1, amount1);
            let cost: i64 = price1 * i1 + price2 * i2;
            min_cost = min_cost.min(cost);
        }
        min_budget += min_cost;
    }
    min_budget
}

// div_ceil(n, m) = n / m
fn div_ceil(n: i64, m: i64) -> i64 {
    (n + (m - 1)) / m
}