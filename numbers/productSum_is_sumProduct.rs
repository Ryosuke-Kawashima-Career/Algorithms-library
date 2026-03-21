use proconio::input;
const MOD: i64 = 998244353;
// arc107a
// 線形性: 積の和は分配法則で和の積にする
// Q. ΣΣΣ(a * b * c)
// A. (1 + .. + a)(1 + .. + b)(1 + .. + c)
fn main() {
    input!{a: i64, b: i64, c: i64}
    let ans = ((sum(a) * sum(b)) % MOD * sum(c)) % MOD;
    println!("{}", ans);
}

fn sum(num: i64) -> i64 {
    // Σi=1..=n(i) = (1 + n) * n / 2
    ((1 + num) % MOD * (num % MOD) / 2) % MOD
}