use proconio::input;
use std::collections::HashMap;
// ABC446D
// Q. a[]: array of integers, what is the longest subsequence of b[] such that b[i] + 1 == b[i+1]?
// A. DP just like solving Longest Increasing Subsequence (LIS)
fn main() {
    input! {n: usize, a: [usize; n]}
    // dp[last number] = length
    let mut dp: HashMap<usize, usize> = HashMap::new();
    for &num in a.iter() {
        let last_num: usize = num - 1;
        if let Some(&prev_len) = dp.get(&last_num) {
            let cur_len: usize = prev_len + 1;
            dp.entry(num)
                .and_modify(|length| *length = (*length).max(cur_len))
                .or_insert(cur_len);
        } else {
            dp.entry(num).or_insert(1);
        }
    }
    let max_length: usize = *dp.values().max().unwrap_or(&0);
    println!("{}", max_length);
}
