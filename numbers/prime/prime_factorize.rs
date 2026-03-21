use proconio::input;
// abc383d
// Q. N 以下の正整数のうち、正の約数をちょうど9個持つものの個数
// A. 素因数分解する．p1^r1 * p2^r2の因数の個数は(r1+1)*(r2+1)つまり，p^8とp^2*q^2
// p,q については O(√N) 以下の素数のみ考えればよいです。pを昇順に全探索していきます。このときp^2*q^2≤Nなるqの最大値は単調に減少していくので、尺取り法を用いることで、O(√N) 以下の素数の個数に対して線形時間でこの問題を解く
fn main() {
    input!{n: usize}
    let primes: Vec<usize> = enum_primes(1000005);
    let mut ans: usize = 0;
    // 全探索を用いる
    for &p in primes.iter() {
        if p * p * p * p > n {
            break;
        }
        // anti-overflow
        if p < 100 {
            if p * p * p * p * p * p * p * p <= n {
                ans += 1;
            }
        }
        for &q in primes.iter() {
            if q <= p {
                continue;
            }
            if p * p * q * q <= n {
                ans += 1;
            } else {
                break;
            }
        }
    }
    println!("{}", ans);
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