use proconio::{input, marker::Chars};
// ABC447C
// Q. The string s is transformed by adding and deleting 'A's. Find the minimum number of operations to transform s to t.
// A. 1: Consider the strings without 'A'
//    2: Consider the numbers of 'A's.
// 問題を特別な要素に関して分割する
fn main() {
    input! {s: Chars, t: Chars}
    let slen: usize = s.len();
    let tlen: usize = t.len();
    let (s_non_a, s_a): (Vec<char>, Vec<usize>) = decompose(&s);
    let (t_non_a, t_a): (Vec<char>, Vec<usize>) = decompose(&t);
    if &s_non_a != &t_non_a || s_a.len() != t_a.len() {
        println!("-1");
        return;
    }
    let mut operations: usize = 0;
    for i in 0..s_a.len() {
        operations += s_a[i].abs_diff(t_a[i]);
    }
    println!("{}", operations);
}

fn decompose(s: &Vec<char>) -> (Vec<char>, Vec<usize>) {
    /* Transforms s into the string deprived of 'A' and the array of the numbers of consecutive 'A's.
     * For example, if s = ['A', 'B', 'A', 'A', 'C', 'A'], it returns (['B', 'C'], [1, 2, 1])
     */
    let mut non_a: Vec<char> = Vec::new();
    let mut a_counts: Vec<usize> = Vec::new();
    let mut cur_a_count: usize = 0;
    for &c in s.iter() {
        if c == 'A' {
            cur_a_count += 1;
        } else {
            a_counts.push(cur_a_count);
            cur_a_count = 0;
            non_a.push(c);
        }
    }
    a_counts.push(cur_a_count);
    (non_a, a_counts)
}
