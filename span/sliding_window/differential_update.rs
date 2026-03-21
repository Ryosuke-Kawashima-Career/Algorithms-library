use proconio::{input, marker::Chars};
// ABC449C
// Q. Lenght of the string is l <= length <= r. What is the number of substrings which meet s[i] == s[j]
// A. Differential update => Sliding window
fn main() {
    // l or r: length
    input! {n: usize, l: usize, r: usize, s: Chars}
    let s_nums: Vec<usize> = s.iter().map(|&c| c as usize - 'a' as usize).collect();
    let mut count: Vec<usize> = vec![0; 26];
    let r = r + 1;
    // j - R <= i <= j - L
    let mut ans: usize = 0;
    for i in 0..n {
        // i is the right end of the substring
        // entry condition
        if i >= l {
            count[s_nums[i - l]] += 1;
        }
        // exit condition
        if i >= r {
            count[s_nums[i - r]] -= 1;
        }
        ans += count[s_nums[i]];
    }
    println!("{}", ans);
}
