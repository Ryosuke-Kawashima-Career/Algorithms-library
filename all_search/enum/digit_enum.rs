use proconio::input;
use std::collections::HashSet;
// 鉄則C9
// 再帰関数で全探索する
fn main() {
    input!{n: usize, st: [(usize, usize); n]}
    let numbers: Vec<usize> = enum_all(4);
    let mut numbers: HashSet<usize> = numbers.iter().cloned().collect();
    
    for &(number, t) in st.iter() {
        let mut digits = calc_digits(number);
        match t {
            1 => {
                // 正しい
                println!("{}", number);
                return;
            },
            2 => {
                // 1つの桁が違う
                let candidates1 = enum_ok(&mut digits, 1, 0);
                let candidates1: HashSet<usize> = candidates1.iter().cloned().collect();
                // 答えとなる数を集合の積で求める
                numbers = numbers.intersection(&candidates1).copied().collect();
            },
            _ => {
                let mut candidates: HashSet<usize> = HashSet::new();
                // 候補を集合の和で求める
                for diff_digit in 2..=4 {
                    let candidates_part = enum_ok(&mut digits, diff_digit, 0);
                    let candidates_part: HashSet<usize> = candidates_part.iter().cloned().collect();
                    candidates = candidates.union(&candidates_part).copied().collect();
                }
                numbers = numbers.intersection(&candidates).copied().collect();
            },
        }
    }

    if numbers.len() == 1 {
        println!("{:0>4}", numbers.iter().next().unwrap());
    } else {
        // 番号を一つに特定できない
        println!("Can't Solve");
    }
}

fn enum_all(digits: usize) -> Vec<usize> {
    let mut res: Vec<usize> = Vec::new();
    // base case
    if digits == 0 {
        res.push(0);
        return res;
    }

    let nums = enum_all(digits-1);
    for &num in nums.iter() {
        for digit in 0..10 {
            res.push(10 * num + digit);
        }
    }
    return res;
}

// digits: 現在の数を記録する4桁の番号, diff: 答えと何桁違うか?, cur_index: 数字を変える桁
fn enum_ok(digits: &mut Vec<usize>, diff: usize, cur_index: usize) -> Vec<usize> {
    let mut res: Vec<usize> = Vec::new();
    // base case
    if diff == 0 {
        let mut number: usize = 0;
        for i in 0..4 {
            number *= 10;
            number += digits[i];
        }
        res.push(number);
        return res;
    }
    if cur_index == 4 {
        return res;
    }

    let no_changes = enum_ok(digits, diff, cur_index + 1);
    res.extend(no_changes);
    // 復元のために元の値を保存する
    let cur_digit: usize = digits[cur_index];
    for digit in 0..10 {
        if digit != cur_digit {
            digits[cur_index] = digit;
            let changed = enum_ok(digits, diff-1, cur_index+1);
            res.extend(changed);
            // 再帰関数の帰りがけに!元の数を復元する
            digits[cur_index] = cur_digit;
        }
    }

    return res;
}

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