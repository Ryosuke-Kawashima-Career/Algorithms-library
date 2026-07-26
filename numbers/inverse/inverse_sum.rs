use proconio::input;
const MOD: i64 = 998244353;
fn main() {
    input! {n: usize};
    let mut inv_sum: Vec<i64> = vec![0; n + 1];
    for num in 1..=n {
        let inv_num: i64 = mod_inv(num as i64);
        inv_sum[num] = (inv_sum[num - 1] + inv_num) % MOD;
    }
    println!("{}", inv_sum[n]);
}
fn mod_inv(num: i64) -> i64 {
    mod_pow(num, MOD - 2)
}

fn mod_pow(base: i64, exp: i64) -> i64 {
    if exp == 0 {
        return 1;
    }
    let base_square: i64 = (base * base) % MOD;
    if exp % 2 == 1 {
        return (base * mod_pow(base_square, exp / 2)) % MOD;
    } else {
        return mod_pow(base_square, exp / 2) % MOD;
    }
}
