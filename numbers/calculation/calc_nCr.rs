use proconio::input;
const MOD: usize = 1_000_000_007;
// 鉄則A30: nCr
fn main() {
    input!{n: usize, r: usize}
    let mut fact: Vec<usize> = vec![1; n+1];
    // a ^ p == 1 (mod p) i.e. a ^ (p-2) = inv(a)
    let mut inv_fact: Vec<usize> = vec![1; n+1];
    for num in 1..=n {
        fact[num] = num * fact[num-1];
        fact[num] %= MOD;
        inv_fact[num] = pow_mod(fact[num], MOD-2);
    }
    // %と*の優先度は同じらしい
    println!("{}", fact[n] * inv_fact[r] % MOD * inv_fact[n-r] % MOD)
}

fn pow_mod(n: usize, m: usize) -> usize {
    if m == 0 {
        return 1;
    }

    let n_sq: usize = (n * n) % MOD;
    let res: usize = if m % 2 == 0 {
        pow_mod(n_sq, m / 2) % MOD
    } else {
        let prev: usize = pow_mod(n_sq, (m - 1) / 2) % MOD;
        n * prev % MOD
    };

    res % MOD
}