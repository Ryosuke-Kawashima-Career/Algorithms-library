use proconio::input;
use std::collections::HashSet;
// ABC439B:Happy Number
// Q. A Happy Number is a number that eventually reaches 1 when replaced by the sum of the square of each digit.
// A. Monotonous Decreasing: 対象の数は次第に減少するので，loopを繰り返せば勝手に値が収束する．
fn main() {
    input! {n: usize}
    let mut seen = HashSet::new();
    seen.insert(n);
    let mut is_happy = true;
    let mut current = n;
    while current != 1 {
        current = calc_happy(current);
        if seen.contains(&current) {
            is_happy = false;
            break;
        }
        seen.insert(current);
    }
    if is_happy {
        println!("Yes");
    } else {
        println!("No");
    }
}

fn calc_happy(n: usize) -> usize {
    let n_digits: Vec<usize> = n
        .to_string()
        .chars()
        .map(|c| c.to_digit(10).unwrap() as usize)
        .collect();
    let mut happy: usize = 0;
    for digit in n_digits {
        happy += digit * digit;
    }
    happy
}
