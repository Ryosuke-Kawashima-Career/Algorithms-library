const MOD: i64 = 1_000_000_007;
// n ^ m
fn pow_mod(n: i64, m: i64) -> i64 {
    if m == 0 {
        return 1;
    }

    let n_sq: i64 = (n * n) % MOD;
    let res: i64 = if m % 2 == 0 {
        pow_mod(n_sq, m / 2) % MOD
    } else {
        let prev: i64 = pow_mod(n_sq, (m - 1) / 2) % MOD;
        n * prev % MOD
    };

    res % MOD
}
