fn get_pow2_pow2inv_mod(n: usize) -> (Vec<i64>, Vec<i64>) {
    // Precompute powers of 2 and inverse powers of 2
    let mut pow2 = vec![1; n + 1];
    let mut inv2 = vec![1; n + 1];
    // 2^(-1) = (MOD + 1) / 2 ∵ 2 * inv2_val == MOD + 1 == 1
    let inv2_val = (MOD + 1) / 2;
    for i in 1..=n {
        pow2[i] = pow2[i - 1] * 2 % MOD;
        inv2[i] = inv2[i - 1] * inv2_val % MOD;
    }
    (pow2, inv2)
}
