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
