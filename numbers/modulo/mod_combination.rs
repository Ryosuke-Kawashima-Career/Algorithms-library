fn comb(n: usize, k: usize) -> usize {
    /* Returns nCk */
    if k > n {
        return 0;
    }
    if k == 0 || k == n {
        return 1;
    }

    let mut numerator: usize = 1;
    for i in 0..k {
        numerator = (numerator * (n - i)) % MOD;
    }
    let mut denominator: usize = 1;
    for i in 1..=k {
        denominator = (denominator * i) % MOD;
    }
    let denominator_inv: usize = mod_inverse(denominator, MOD);
    // n! * (k!)^(-1) * ((n - k)!)^(-1)
    (numerator * denominator_inv) % MOD
}

fn power(m: usize, n: usize) -> usize {
    if n == 0 {
        return 1;
    }

    let m_square: usize = m * m % MOD;
    let result: usize = if n % 2 == 0 {
        power(m_square, n / 2)
    } else {
        (m * power(m_square, (n - 1) / 2)) % MOD
    };
    result % MOD
}

fn mod_inverse(n: usize, m: usize) -> usize {
    power(n, m - 2)
}

fn comb_functions(max_n: usize) {
    let mut fact = vec![1u64; max_n + 1];
    let mut inv = vec![1u64; max_n + 1];

    for i in 1..=max_n {
        fact[i] = (fact[i - 1] * i as u64) % MOD;
    }

    inv[max_n] = power(fact[max_n], MOD - 2);
    for i in (1..max_n).rev() {
        inv[i] = (inv[i + 1] * (i + 1) as u64) % MOD;
    }
    let ncr = |n: usize, r: usize| -> u64 {
        if r > n {
            return 0;
        }
        let num = fact[n];
        let den = (inv[r] * inv[n - r]) % MOD;
        (num * den) % MOD
    };
}
