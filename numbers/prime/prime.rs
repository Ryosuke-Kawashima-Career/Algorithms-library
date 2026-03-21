fn is_prime(num: i64) -> bool {
    let mut cur: i64 = 2;
    while cur * cur <= num {
        if num % cur == 0 {
            return false;
        }
        cur += 1;
    }

    return true;
}

fn eratosthenes(max_num: usize) -> Vec<bool> {
    let mut is_prime: Vec<bool> = vec![true; max_num+1];
    is_prime[0] = false;
    is_prime[1] = false;
    for num in 2..=max_num {
        if !is_prime[num] {
            continue;
        }
        let mut k: usize = 2;
        while k * num <= max_num {
            is_prime[k*num] = false;
            k += 1;
        }
    }

    return is_prime;
}

fn enum_primes(max_num: usize) -> Vec<usize> {
    let mut is_prime: Vec<bool> = vec![true; max_num+1];
    let mut primes: Vec<usize> = Vec::new();
    is_prime[0] = false;
    is_prime[1] = false;
    for num in 2..=max_num {
        if !is_prime[num] {
            continue;
        }
        primes.push(num);
        let mut k: usize = 2;
        while k * num <= max_num {
            is_prime[k * num] = false;
            k += 1;
        }
    }

    return primes;
}

// それぞれの数に対する素因数の数を計算する。
fn prime_factors(n: usize) -> Vec<usize> {
    let mut prime_factor_nums: Vec<usize> = vec![0; n+1];

    for num in 2..=n {
        // 素因数を持たない(0, 1)もしくは、素数か
        if prime_factor_nums[num] == 0 {
            let mut k: usize = 1;
            while k * num <= n {
                prime_factor_nums[k*num] += 1;
                k += 1;
            }
        }
    }

    return prime_factor_nums;
}

fn count_primes(num: usize) -> usize {
    let mut n: usize = num;
    let mut div: usize = 2;
    let mut cnt: usize = 0;

    while div * div <= num {
        // 素数divで割り切れるまで計算する
        while n % div == 0 {
            // nを素数で割って小さくし、4などの素数以外の数で割れないようにする!!!
            n /= div;
            cnt += 1;
        }
        div += 1;
    }

    // numが素数のとき(corner case)
    if n > 1 {
        cnt += 1;
    }
    return cnt;
}

fn enum_prime_factors(n: usize) -> Vec<usize> {
    let mut num: usize = n;
    let mut div: usize = 2;
    let mut res: Vec<usize> = Vec::new();

    while num > 1 && div * div <= n {
        while num % div == 0 {
            res.push(div);
            num /= div;
        }
        div += 1;
    }
    if num > 1 {
        res.push(num);
    }

    res
}