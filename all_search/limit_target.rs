use proconio::input;
// 典型85
// 全探索の範囲を絞る。
// 変数の数を制限する。
fn main() {
    // a*b*c = k となる(a,b,c)の個数
    input!{k: i64}
    let mut divisors: Vec<i64> = Vec::new();
    let mut div: i64 = 1;
    while div * div <= k {
        if k % div == 0 {
            divisors.push(div);
            if div != k / div {
                divisors.push(k / div);
            }
        }
        div += 1;
    }
    divisors.sort();

    // 約数の組み合わせ
    let mut ans: usize = 0;
    for i in 0..divisors.len() {
        for j in i..divisors.len() {
            let divisor: i64 = k / divisors[i] / divisors[j];
            if divisor > 0 
            && divisor >= divisors[i] && divisor >= divisors[j] 
            && k == divisor * divisors[i] * divisors[j] {
                ans += 1;
            }
        }
    }

    println!("{}", ans);
}