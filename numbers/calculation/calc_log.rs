use proconio::input;
// 典型75
// log and prime
fn main() {
    input!{n: usize}
    let parts: usize = primes(n);
    // exp = 2^pow
    let mut pow: usize = 0;
    let mut exp: usize = 1;
    // equal to: pow = log2(parts)
    while exp <= parts {
        if exp == parts {
            break;
        }
        pow += 1;
        exp *= 2;
    }
    println!("{}", pow);
}

fn primes(num: usize) -> usize {
    let mut n: usize = num;
    let mut div: usize = 2;
    let mut cnt: usize = 0;
    while div * div <= num {
        // 割りきれる限り割る
        while n % div == 0 {
            cnt += 1;
            n /= div;
        }
        div += 1;
    }
    if n > 1 {
        cnt += 1;
    }
    return cnt;
}