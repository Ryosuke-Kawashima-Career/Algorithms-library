use proconio::input;
// ABC406C
// Q. you are given a permutation of (1,2,...,N). Find the number of (contiguous) subarrays of P that are tilde-shaped. tilde-shape is defined as follows: (~: tilde)
// - The length |A| is at least 4.
// - A1 < A2.
// - There exists exactly one integer i with 2 ≤ i < |A| such that Ai-1 < Ai > Ai+1.
// - There exists exactly one integer i with 2 ≤ i < |A| such that Ai-1 > Ai < Ai+1.
// A. Subarray: Dynamic Programming
// A. ~を部分列で分解する
fn main() {
    input! {n: usize, p: [usize; n]}
    // dp_ascend[i]: 添字 i を終点とする連続増加部分列（長さ2以上）の個数
    let mut dp_ascend: Vec<usize> = vec![0; n];
    // dp_mountain[i]: 添字 i を終点とする「山型^」部分列（長さ 3 以上）の個数
    let mut dp_mountain: Vec<usize> = vec![0; n];
    // dp_tilde[i]: 添字 i を終点とする「波型~」部分列（長さ 4 以上）の個数
    let mut dp_tilde: Vec<usize> = vec![0; n];
    for i in 1..n {
        if p[i - 1] < p[i] {
            dp_ascend[i] = dp_ascend[i - 1] + 1;
            dp_mountain[i] = 0;
            dp_tilde[i] = dp_tilde[i - 1] + dp_mountain[i - 1];
        }
        if p[i - 1] > p[i] {
            dp_ascend[i] = 0;
            dp_mountain[i] = dp_mountain[i - 1] + dp_ascend[i - 1];
            dp_tilde[i] = 0;
        }
    }
    let ans: usize = dp_tilde.iter().sum();
    println!("{}", ans);
}
