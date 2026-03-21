use proconio::input;
const MOD: i64 = 998244353;
const MAX_NUM: i64 = 10_000_005;

#[derive(Debug, Copy, Clone, Default)]
struct PrimeStats {
    max1: i64,
    max2: i64,
    max1_count: usize,
    who_is_max1: usize,
}
// Q. Find the LCM of A \ {A_k} mod M
// A. Calculate the maximum and the second maximum exponent of each prime factor
// Key Point: If the maximum exponent of a prime factor p is achieved by only one element A_k,
// then the exponent of p in the LCM of A \ {A_k} is the second maximum exponent of p.
// Otherwise, the exponent of p in the LCM of A \ {A_k} is the maximum exponent of p.
fn main() {
    // Precompute SPF(Smallest Prime Factor)
    // MAX_NUM is i64, need cast for sieve
    let spf = sieve(MAX_NUM);

    // Reuse allocations
    // Need +1 for 1-based indexing if needed, or just 0..MAX_NUM
    let mut prime_stats = vec![PrimeStats::default(); (MAX_NUM + 1) as usize];
    // Pre-allocate visited_primes to avoid re-allocation
    let mut visited_primes = Vec::with_capacity(10000);
    let mut is_visited = vec![false; (MAX_NUM + 1) as usize];

    input! {t: usize}
    for _ in 0..t {
        input! {n: usize, a: [i64; n]}

        // 1. Collect Stats
        get_prime_stats(
            &a,
            &spf,
            &mut prime_stats,
            &mut visited_primes,
            &mut is_visited,
        );

        // 2. Compute Total LCM
        let mut total_lcm: i64 = 1;
        // OPTIMIZATION: Iterate only visited primes
        for &prime in &visited_primes {
            let stats = &prime_stats[prime as usize];
            if stats.max1 > 0 {
                total_lcm = total_lcm * modpow(prime, stats.max1) % MOD;
            }
        }

        // 3. Compute Answers
        let lcm_without_ak = get_lcm_without_ak(&a, &prime_stats, &visited_primes, total_lcm);

        for k in 0..n {
            print!("{} ", lcm_without_ak[k]);
        }
        println!();

        // 4. Cleanup
        for &p in &visited_primes {
            prime_stats[p as usize] = PrimeStats::default();
            is_visited[p as usize] = false;
        }
        visited_primes.clear();
    }
}

fn get_prime_stats(
    a: &[i64],
    spf: &[i64],
    prime_stats: &mut Vec<PrimeStats>,
    visited_primes: &mut Vec<i64>,
    is_visited: &mut Vec<bool>,
) {
    for (i, &val) in a.iter().enumerate() {
        let mut num = val;
        while num > 1 {
            let p = spf[num as usize] as i64;
            let mut exp = 0;
            while num % p == 0 {
                num /= p;
                exp += 1;
            }

            if !is_visited[p as usize] {
                is_visited[p as usize] = true;
                visited_primes.push(p);
            }

            let stats = &mut prime_stats[p as usize];
            if exp > stats.max1 {
                stats.max2 = stats.max1;
                stats.max1 = exp;
                stats.max1_count = 1;
                stats.who_is_max1 = i;
            } else if exp == stats.max1 {
                stats.max1_count += 1;
            } else if exp > stats.max2 {
                stats.max2 = exp;
            }
        }
    }
}

fn get_lcm_without_ak(
    a: &[i64],
    prime_stats: &Vec<PrimeStats>,
    visited_primes: &Vec<i64>,
    total_lcm: i64,
) -> Vec<i64> {
    let n = a.len();
    let mut ans = vec![total_lcm; n];

    // Check reduction for each unique max contributor
    for &p in visited_primes {
        let stats = &prime_stats[p as usize];
        if stats.max1_count == 1 {
            let k = stats.who_is_max1;
            let diff = stats.max1 - stats.max2;
            if diff > 0 {
                let reduction = modpow(p, diff);
                let inv_red = get_modinv(reduction);
                ans[k] = ans[k] * inv_red % MOD;
            }
        }
    }
    ans
}

fn sieve(n: i64) -> Vec<i64> {
    let mut spf = vec![0; (n + 1) as usize];
    for i in 0..=n as usize {
        spf[i] = i as i64;
    }
    for i in 2..=(n as usize) {
        if spf[i] == i as i64 {
            let mut j = 2 * i;
            while j <= n as usize {
                if spf[j] == j as i64 {
                    spf[j] = i as i64;
                }
                j += i;
            }
        }
    }
    spf
}

fn modpow(mut base: i64, mut exp: i64) -> i64 {
    let mut res = 1;
    base %= MOD;
    while exp > 0 {
        if exp % 2 == 1 {
            res = res * base % MOD;
        }
        base = base * base % MOD;
        exp /= 2;
    }
    res
}

fn get_modinv(n: i64) -> i64 {
    modpow(n, MOD - 2)
}
