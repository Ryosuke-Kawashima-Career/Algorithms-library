use proconio::input;
use std::collections::HashMap;
//abc439C: Root Brute Force
// Q. A positive integer n is called a good integer when it satisfies the following condition: There exists exactly one pair of integers (x,y) that satisfies 0<x<y and x^2+y^2 =n.
// Q. Find all good integers not greater than n.
// A. N < 10^7なのでO(√n)を疑う．つまり，平方数の要素を列挙する
fn main() {
    input! {n: usize}
    let elements: Vec<usize> = enum_elements(n);
    let n_elements: usize = elements.len();
    let mut good_numbers: HashMap<usize, usize> = HashMap::new();
    for i1 in 0..n_elements {
        for i2 in i1 + 1..n_elements {
            let candidate: usize = elements[i1] * elements[i1] + elements[i2] * elements[i2];
            if candidate > n {
                break;
            }
            *good_numbers.entry(candidate).or_insert(0) += 1;
        }
    }
    print_good_numbers(good_numbers);
}

fn enum_elements(n: usize) -> Vec<usize> {
    let mut num: usize = 1;
    let mut elements: Vec<usize> = Vec::new();
    while num * num < n {
        elements.push(num);
        num += 1;
    }
    return elements;
}

fn print_good_numbers(good_numbers_count: HashMap<usize, usize>) {
    let mut good_numbers: Vec<usize> = Vec::new();
    for (&good_number, &count) in good_numbers_count.iter() {
        if count == 1 {
            good_numbers.push(good_number);
        }
    }
    good_numbers.sort();
    let k: usize = good_numbers.len();
    println!("{}", k);
    for good_number in good_numbers {
        print!("{} ", good_number);
    }
    println!("");
}
