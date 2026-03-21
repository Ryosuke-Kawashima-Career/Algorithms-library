use proconio::input;
const MOD: i64 = 1_000_000_007;
// 典型52
// 約数の和などに応用できる。ex. 18の約数の和: (1 + 2) * (1 + 3 + 9)
// 組み合わせの総和
fn main() {
    input!{n: usize, a: [[i64; 6]; n]}
    let mut comb_sum: i64 = 1;

    for i in 0..n {
        let part_sum: i64 = a[i].iter().sum();
        comb_sum *= part_sum;
        comb_sum %= MOD;
    }

    println!("{}", comb_sum % MOD);
}