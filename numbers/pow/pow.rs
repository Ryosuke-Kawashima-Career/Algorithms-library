fn main() {
    let number: i64 = 10;
    let mut pows: Vec<i64> = vec![1; n+1];
    for i in 1..=n {
        pows[i] = number*pows[i-1];
    }
    let mut prefix_pow: Vec<i64> = vec![0; n+1];
    for i in 1..=n {
        prefix_pow[i] = prefix_pow[i-1] + pows[i-1];
    }
}

const MOD: i64 = 1_000_000_007;

fn pow(a: i64, b: usize) -> i64 {
    let mut res: i64 = 1;
    if b == 0 {
        return 1;
    }

    if b % 2 == 0 {
        res *= pow((a * a) % MOD, b / 2) % MOD;
        res %= MOD;
    } else {
        res *= (a * (pow((a * a) % MOD, (b - 1) / 2) % MOD)) % MOD;
        res %= MOD;
    }

    return res % MOD;
}