const MOD: i64 = 998244353;

struct Comb {
    fact: Vec<i64>,
    fact_inv: Vec<i64>,
}

impl Comb {
    fn new(max_n: usize) -> Self {
        let mut fact = vec![1i64; max_n + 1];
        for i in 1..=max_n {
            fact[i] = fact[i - 1] * i as i64 % MOD;
        }
        let mut fact_inv = vec![1i64; max_n + 1];
        fact_inv[max_n] = pow_mod(fact[max_n], MOD - 2);
        for i in (1..=max_n).rev() {
            fact_inv[i - 1] = fact_inv[i] * i as i64 % MOD;
        }
        Self { fact, fact_inv }
    }

    // C(n, r) mod MOD, safe for any n, r (returns 0 when out of range).
    fn ncr(&self, n: i64, r: i64) -> i64 {
        if r < 0 || n < 0 || r > n {
            return 0;
        }
        let (n, r) = (n as usize, r as usize);
        self.fact[n] * self.fact_inv[r] % MOD * self.fact_inv[n - r] % MOD
    }
}

fn pow_mod(mut base: i64, mut exp: i64) -> i64 {
    base %= MOD;
    let mut result: i64 = 1;
    while exp > 0 {
        if exp & 1 == 1 {
            result = result * base % MOD;
        }
        base = base * base % MOD;
        exp >>= 1;
    }
    result
}
