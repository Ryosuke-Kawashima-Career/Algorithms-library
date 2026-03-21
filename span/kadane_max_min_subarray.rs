use proconio::{input, marker::Chars};
// ABC 408 D
// Theme: transforming a "state manipulation" problem into a "maximum/minimum subarray sum" problem.
// Kadane's Algorithm (usually used for max subarray sum)
// Q. Change some 0s to 1s to minimize the number of 1s in the string.
// 10011 -> 00011
// A. Prefix sums + greedy | Count 0 as +1, 1 as -1
// cost(L,R)=(#zeros inside [L,R))+(#ones outside [L,R)).
// = (flip 0 to 1 inside [L,R)) + (flip 1 to 0 outside [L,R))
/*
Cost = zeros_inside + ones_outside
        = zeros_inside + (total_ones - ones_inside)
        = (zeros_inside - ones_inside) + total_ones
 */
fn main() {
    input!{t: usize}
    for _ in 0..t {
        input!{n: usize, s: Chars}
        // count 0 as +1, 1 as -1
        let mut signs: Vec<i64> = vec![0; n+1];
        // find min signs[j] - max signs[i] (i < j)
        for i in 1..=n {
            signs[i] = signs[i-1] + if s[i-1] == '0' { 1 } else { -1 };
        }
        let mut max_sign: i64 = 0;
        let mut ans: i64 = 0;
        for i in 0..=n {
            ans = ans.min(signs[i] - max_sign);
            max_sign = max_sign.max(signs[i]);
        }
        // the cost of turning the entire string into all zeros.cost(0, n)
        let total_ones: i64 = s.iter().filter(|&&c| c == '1').count() as i64;
        println!("{}", ans + total_ones);
    }
}