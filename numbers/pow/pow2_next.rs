// abc337e
/*  /algo/identify_binary_bit.rs */
fn main() {
    // m = log2(n) <=> 2^m = n
    // (power, exponential)
    let (m, exp) = next_pow2(n);
}

// 2 ^ m >= n
#[allow(dead_code)]
fn next_pow2(n: usize) -> (usize, usize) {
    let mut m: usize = 0;
    while (1 << m) < n {
        m += 1;
    }
    return (m, 1 << m);
}
#[allow(dead_code)]
fn next_power2(n: usize) -> (usize, usize) {
    // 2 ^ pow == exp
    let mut exp: usize = 1;
    let mut pow: usize = 0;
    while exp < n {
        pow += 1;
        exp <<= 1;
    }
    return (pow, exp);
}