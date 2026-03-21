// usize -> u128, i64
fn gcd(m: i64, n: i64) -> i64 {
    if m == 0 || n == 0 {
        return m + n;
    }

    gcd(n, m % n)
}

// 割り算を掛け算の前に行うことでoverflowを防ぐ。
// lcmの式変形: lcm = a * b / gcd > LIMIT <=> a / gcd > LIMIT / b
fn lcm(m: i64, n: i64) -> i64 {
    m / gcd(m, n) * n
}

// 約数の組を列挙 ex. 4: 1*4, 2*2
fn enum_div_pair(n: usize) -> Vec<(usize, usize)> {
    let mut res: Vec<(usize, usize)> = Vec::new();
    let mut i: usize = 1;
    while i * i <= n {
        if n % i == 0 {
            let j: usize = n / i;
            res.push((i, j));
        }
        i += 1;
    }

    return res;
}

fn enum_divisors(num: i64) -> Vec<i64> {
    let mut divisors: Vec<i64> = Vec::new();
    let mut div: i64 = 1;
    while div * div <= num {
        if num % div == 0 {
            divisors.push(div);
            if div != num / div {
                divisors.push(num / div);
            }
        }
        div += 1;
    }
    divisors.sort();

    return divisors;
}