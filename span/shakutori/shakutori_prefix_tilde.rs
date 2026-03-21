use proconio::input;
// ABC406C
// Q. you are given a permutation of (1,2,...,N). Find the number of (contiguous) subarrays of P that are tilde-shaped. tilde-shape is defined as follows: (~: tilde)
// - The length |A| is at least 4.
// - A1 < A2.
// - There exists exactly one integer i with 2 ≤ i < |A| such that Ai-1 < Ai > Ai+1.
// - There exists exactly one integer i with 2 ≤ i < |A| such that Ai-1 > Ai < Ai+1.
// A. 尺取り法
fn main() {
    input! {
        n: usize,
        p: [usize; n],
    }

    if n < 4 {
        println!("0");
        return;
    }

    // left_up[i] = length of the contiguous increasing subsequence ending at i
    let mut left_up = vec![0; n];
    left_up[0] = 1;
    for i in 1..n {
        if p[i] > p[i - 1] {
            left_up[i] = left_up[i - 1] + 1;
        } else {
            left_up[i] = 1;
        }
    }

    // right_up[i] = length of the contiguous increasing subsequence starting at i
    let mut right_up = vec![0; n];
    right_up[n - 1] = 1;
    for i in (0..n - 1).rev() {
        if p[i] < p[i + 1] {
            right_up[i] = right_up[i + 1] + 1;
        } else {
            right_up[i] = 1;
        }
    }

    let mut ans: usize = 0;

    // Iterate to find all maximal decreasing segments
    let mut i = 0;
    while i < n - 1 {
        if p[i] > p[i + 1] {
            let l = i;
            // Extend the decreasing segment
            while i < n - 1 && p[i] > p[i + 1] {
                i += 1;
            }
            let r = i;

            // We found a decreasing segment p[l] > p[l+1] > ... > p[r].
            // If l is a peak and r is a valley, then we can extend left and right.
            // left_up[l] > 1 ensures p[l-1] < p[l] (Peak)
            // right_up[r] > 1 ensures p[r] < p[r+1] (Valley)
            // The number of valid l's is left_up[l] - 1.
            // The number of valid r's is right_up[r] - 1.
            // つまり，~の中央の減少部分を元に計算するのか...

            ans += (left_up[l] - 1) * (right_up[r] - 1);
        } else {
            i += 1;
        }
    }

    println!("{}", ans);
}
