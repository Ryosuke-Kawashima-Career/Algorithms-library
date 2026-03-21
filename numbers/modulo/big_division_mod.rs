use proconio::input;
const P: usize = 10007;
// ABC448E
// Q. Calculate N mod M (N is represented by Run-Length Encoding)
// A. Formula (N = floor(N/M) * M + N % M) + Modular Multiplicative Inverse
fn main() {
    input! {
        k: usize,
        m: usize,
        cl: [(usize, usize); k],
    }

    // 1. Calculate N mod M  (R_M)
    let r_m = calc_mod(m, &cl);

    // 2. Calculate N mod 10007 (R_P)
    let r_p = calc_mod(P, &cl);

    // 3. Find the Modular Multiplicative Inverse of M modulo 10007
    // Using Fermat's Little Theorem: M^(P-2) mod P
    let m_inv = power(m, P - 2, P);

    // 4. Calculate Final Answer: (R_P - R_M) * M_inv mod P
    // Adding P to R_P ensures we don't underflow below 0 when subtracting
    let ans = ((r_p + P - (r_m % P)) * m_inv) % P;

    println!("{}", ans);
}

// Iterates through the run-length encoding blocks to find N mod [md]
fn calc_mod(md: usize, cl: &[(usize, usize)]) -> usize {
    let mut s: usize = 0;
    for &(c, l) in cl {
        let shift_factor = power(10, l, md);
        let block_value = geo_sum(10, l, md);

        // Shift the running total to the left by L digits (equivalent to multiplying by 10^L)
        s = (s * shift_factor) % md;

        // Add the mathematical value of the new block
        s = (s + c * block_value) % md;
    }
    s
}

// Computes classic Modular Exponentiation: (base^exp) mod md
fn power(mut base: usize, mut exp: usize, md: usize) -> usize {
    let mut res = 1 % md;
    base %= md;
    while exp > 0 {
        if exp % 2 == 1 {
            res = (res * base) % md;
        }
        base = (base * base) % md;
        exp /= 2;
    }
    res
}

// Computes the Geometric Series modulo [md] cleanly without using division!
// Sum = (a^0 + a^1 + a^2 + ... + a^{l-1}) mod md
// This avoids the fatal "division by 9" panic if [md] happens to be a multiple of 3 or 9.
fn geo_sum(a: usize, l: usize, md: usize) -> usize {
    if l == 0 {
        return 0;
    }
    if l % 2 == 1 {
        // If length is odd, compute for (L-1) and add the largest power term at the end
        return (a * geo_sum(a, l - 1, md) + 1) % md;
    }
    // If length is even, we can split it exactly in half
    // Ex: (1 + a + a^2 + a^3) = (1 + a) * (1 + a^2)
    let half_sum = geo_sum(a, l / 2, md);
    let power_shift = power(a, l / 2, md);

    (half_sum * (1 + power_shift)) % md
}
