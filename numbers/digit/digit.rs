// 復元のために桁の順序をもとに戻す
fn calc_digits(mut num: usize) -> Vec<usize> {
    let mut digits: Vec<usize> = Vec::new();
    for _ in 0..4 {
        digits.push(num % 10);
        num /= 10;
    }
    digits.reverse();
    return digits;
}

fn restore_from_digits(digits: &Vec<usize>) -> usize {
    let mut number: usize = 0;
    let n: usize = digits.len();
    for i in 0..n {
        number *= 10;
        number += digits[i];
    }
    return number;
}

fn digits_to_num(digits: &Vec<usize>, base: usize) -> usize {
    // 上の桁から見る
    let mut res: usize = 0;
    for &digit in digits.iter() {
        res *= base;
        res += digit;
    }
    return res;
}