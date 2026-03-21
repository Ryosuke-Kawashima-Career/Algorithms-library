use proconio::input;
use proconio::marker::Chars;
// use a prime number!!!
const MOD: i64 = 1000_000_007;
// 鉄則A56
// 文字列の比較にHashを使用する。
fn main() {
    input!{n: usize, q: usize, s: Chars}
    // 1indexed
    let mut power100: Vec<i64> = vec![1; n+1];
    // 区間クエリだから累積で考える。
    let mut prefix_hash: Vec<i64> = vec![0; n+1];
    let s_nums: Vec<i64> = s.iter().map(|&x| x as i64 - 'a' as i64).collect();

    for i in 1..=n {
        power100[i] = 100 * power100[i-1] % MOD;
        power100[i] %= MOD;
    }

    for i in 1..=n {
        prefix_hash[i] = (prefix_hash[i-1] * 100) % MOD + s_nums[i-1];
        prefix_hash[i] %= MOD;
    }

    // [l r]のhash
    let calc_hash = |l: usize, r: usize| -> i64 {
        let mut val: i64 = prefix_hash[r] - prefix_hash[l-1] * power100[r-l+1] % MOD;
        if val < 0 {
            val += MOD;
        }
        return val;
    };

    for _ in 0..q {
        input!{a: usize, b: usize, c: usize, d: usize}
        let hash1: i64 = calc_hash(a, b);
        let hash2: i64 = calc_hash(c, d);
        if hash1 == hash2 {
            println!("Yes");
        } else {
            println!("No");
        }
    }
}

// 鉄則B56
// Hashの計算の向きを左右2つ用意する
fn palindrome() {
    input!{n: usize, q: usize, s: Chars}
    let s_nums: Vec<i64> = s.iter().map(|&x| x as i64 - 'a' as i64).collect();

    let mut pow100: Vec<i64> = vec![1; n+1];
    for i in 1..=n {
        pow100[i] = pow100[i-1] * 100 % MOD;
        pow100[i] %= MOD;
    }

    let mut hash_left: Vec<i64> = vec![0; n+2];
    let mut hash_right: Vec<i64> = vec![0; n+2];

    for i in 1..=n {
        hash_left[i] = (hash_left[i-1] * 100) % MOD + s_nums[i-1];
        hash_left[i] %= MOD;
        hash_right[n-i+1] = (hash_right[n-i+2] * 100) % MOD + s_nums[n-i];
        hash_right[n-i+1] %= MOD;
    }

    let calc_hash_left = |l: usize, r: usize| -> i64 {
        let mut res = hash_left[r] - hash_left[l-1] * pow100[r - l + 1] % MOD;
        if res < 0 {
            res += MOD;
        }
        res
    };
    let calc_hash_right = |l: usize, r: usize| -> i64 {
        let mut res = hash_right[l] - hash_right[r+1] * pow100[r - l + 1] % MOD;
        if res < 0 {
            res += MOD;
        }
        res
    };

    for _ in 0..q {
        input!{l: usize, r: usize}
        let hash1 = calc_hash_left(l, r);
        let hash2 = calc_hash_right(l, r);
        if hash1 == hash2 {
            println!("Yes");
        } else {
            println!("No");
        }
    }
}

// 文字列を反転する．(前から見るか，後ろから見るか)
fn palindrome_rev() {
    input!{n: usize, q: usize, s: Chars}
    // 左から見たものと右から見たものを用意する．
    let s_nums: Vec<i64> = s.iter().map(|&x| x as i64 - 'a' as i64).collect();
    let s_nums_rev: Vec<_> = s_nums.iter().rev().collect();

    let mut pow100: Vec<i64> = vec![1; n+1];
    for i in 1..=n {
        pow100[i] = pow100[i-1] * 100 % MOD;
        pow100[i] %= MOD;
    }

    let mut hash_left: Vec<i64> = vec![0; n+2];
    let mut hash_right: Vec<i64> = vec![0; n+2];

    for i in 1..=n {
        hash_left[i] = (hash_left[i-1] * 100) % MOD + s_nums[i-1];
        hash_left[i] %= MOD;
        hash_right[i] = (hash_right[i-1] * 100) % MOD + s_nums_rev[i-1];
        hash_right[i] %= MOD;
    }

    let calc_hash = |l: usize, r: usize, hash_prefix: &Vec<i64>| -> i64 {
        let mut res = hash_prefix[r] - hash_prefix[l-1] * pow100[r - l + 1] % MOD;
        if res < 0 {
            res += MOD;
        }
        res
    };

    for _ in 0..q {
        input!{l: usize, r: usize}
        // 向きを反転させる．
        let l_rev: usize = n - r + 1;
        let r_rev: usize = n - l + 1;
        let hash1 = calc_hash(l, r, &hash_left);
        let hash2 = calc_hash(l_rev, r_rev, &hash_right);
        if hash1 == hash2 {
            println!("Yes");
        } else {
            println!("No");
        }
    }
}