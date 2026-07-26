use proconio::input;
// ABC414C: Palindrome
// Q.  Find the sum of all integers between 1 and N, inclusive, whose decimal representation and base-A representation are both palindromes.
// A. Copilot is my best friend!!!
// Mirroring is the key concept of generating palindromes.
fn main() {
    input!{a: usize, n: usize}
    let digits_n: usize = get_digit_numbers(n);
    let palindromes_decimal: Vec<Vec<usize>> = enum_palindromes(digits_n);
    let mut ans: usize = 0;
    for digit in 1..=digits_n {
        for &palindrome in palindromes_decimal[digit].iter() {
            if palindrome <= n && is_palindrome_base_of(a, palindrome) {
                ans += palindrome;
            }
        }
    }
    println!("{}", ans);
}

fn enum_palindromes(digits: usize) -> Vec<Vec<usize>> {
    let mut palindromes: Vec<Vec<usize>> = vec![vec![]; digits + 1];

    // generate palindromes for every length 1..=digits
    for len in 1..=digits {
        let candidates = enum_palindromes_half_half(len);
        for num_chars in candidates {
            if num_chars.is_empty() { continue; }
            if num_chars[0] == '0' { continue; } // skip leading zero numbers
            let mut num: usize = 0;
            for &digit in num_chars.iter() {
                num = num * 10 + digit.to_digit(10).unwrap() as usize;
            }
            palindromes[num_chars.len()].push(num);
        }
    }

    palindromes
}

fn enum_palindromes_half_half(digits: usize) -> Vec<Vec<char>> {
    if digits == 0 {
        return vec![vec![]];
    }
    let half = (digits + 1) / 2;
    // start: for half==1 allow 0..9, otherwise first half must not start with 0
    let start = if half == 1 { 0 } else { 10usize.pow((half - 1) as u32) };
    let end = 10usize.pow(half as u32) - 1;
    let mut palindromes: Vec<Vec<char>> = Vec::new();
    for h in start..=end {
        let half_chars: Vec<char> = h.to_string().chars().collect();
        // safety: skip if string length doesn't match expected half (can happen for pow edge cases)
        if half_chars.len() != half { continue; }
        let mut full = half_chars.clone();
        // if odd digits, skip the middle char when mirroring
        let mirror_start = if digits % 2 == 1 { half_chars.len() - 1 } else { half_chars.len() };
        for i in (0..mirror_start).rev() {
            full.push(half_chars[i]);
        }
        palindromes.push(full);
    }
    palindromes
}

fn get_digit_numbers(n: usize) -> usize {
    let n_string = n.to_string();
    let digits: usize = n_string.chars().count();
    return digits;
}

fn is_palindrome_base_of(base: usize, n: usize) -> bool {
    let mut digits: Vec<usize> = Vec::new();
    let mut num: usize = n;
    while num > 0 {
        digits.push(num % base);
        num /= base;
    }
    let num_digits: usize = digits.len();
    for i in 0..num_digits {
        if digits[i] != digits[num_digits-i-1] {
            return false;
        }
    }
    return true;
}