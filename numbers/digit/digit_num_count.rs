use proconio::input;
// 典型82
// 桁数の求めたかた
fn main() {
    input!{l: i64, r: i64}
    let f = |n: i64| -> i64 {
        n * (n+1) / 2
    };

    // 10^18が最大値なので一つ上の桁まで計算する。
    let mut power10: Vec<i64> = vec![1; 20];
    for pow in 1..=19 {
        power10[pow] = 10 * power10[pow-1]
    }
    let mut ans = 0;

    // 桁数ごとに計算する.
    // i桁の数は10^i-1 <= num < 10^i ex. 10^0 <= 3 <= 10^1 - 1
    for digit in 1..=19 {
        let start: i64 = l.max(power10[digit-1]);
        let end: i64 = r.min(power10[digit]-1);

        if start > end {
            continue;
        }

        let val = f(end) - f(start-1);
        ans += (digit as i64) * val;
    }

    println!("{}", ans);
}