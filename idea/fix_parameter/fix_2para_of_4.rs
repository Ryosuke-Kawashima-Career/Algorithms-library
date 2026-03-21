use proconio::input;
// arc107B
// 2つの変数をまとめて固定
// Q. 1≤a,b,c,d≤N かつ a+b-c-d=K を満たす4つの整数の組(a,b,c,d)の数
fn main() {
    // 足す数と引く数に分類する
    input!{n: i64, k: i64}
    // a+b = k + c+d
    let max_sum_positive: i64 = (k + 2 * n).min(2*n);
    let min_sum_positive: i64 = (k + 2).max(2);

    let mut ans: i64 = 0;
    for sum_positive in min_sum_positive..=max_sum_positive {
        // lower_bound = min.max(), upper_bound = max.min()
        let a_min = 1.max(sum_positive - n);
        let a_max = n.min(sum_positive - 1);
        let num_positive = a_max - a_min + 1;

        let sum_negative = sum_positive - k;
        let c_min = 1.max(sum_negative - n);
        let c_max = n.min(sum_negative - 1);
        let num_negative = c_max - c_min + 1;

        ans += num_positive * num_negative;
    }

    println!("{}", ans);
}