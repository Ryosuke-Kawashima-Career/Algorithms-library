use proconio::input;
const MOD: i64 = 998244353;
// ABC439F
// Q. Count the number of "kadomatsu-like" subsequences in a permutation.
// A. Paraphrase the question = 「門松的な数列=a1<a2 かつ ak−1>ak」
// A. Kadomatsu like shape starts with a mountain and ends in another.
/*
|Variable | Definition | Logic / Condition |
|--- | --- | --- |
|A[L] | Valid start choices | Count of indices j<L such that P[j]<P[L]. |
|B[R] | Valid end choices | Count of indices j>R such that P[j]<P[R]. |
|Middle | Intermediate subsets | "Any subset of elements between index L and R can form the middle. There are 2max(0,R-L-1) such subsets." |
 */
/*
Correct Logic
A subsequence a_1, ... a_k is "kadomatsu-like" (mountains > valleys) if and only if it:

Starts with an ascent: a_1 < a_2
Ends with a descent: a_{k-1} > a_k
This condition implies the sequence of turns starts with a Mountain and ends with a Mountain, ensuring Count(M) = Count(V) + 1.

Algorithm
To count such subsequences efficiently (O(N log N)), we iterate over all possible "start pairs" (a_1, a_2) and "end pairs" (a_{k-1}, a_k).
We compute A and B using a Binary Indexed Tree (Fenwick Tree) and then sum up the contributions:

Case L=R(Subsequence length 3): sum A[i] * B[i]
Case L<R: sum_{L < R} A[L] * B[R] * 2^{R-L-1}. This summation is optimized to O(N) using a running sum of A[L] * 2^{-L}.
 */
// Fenwickで転倒数を数え上げる問題の例題か...
fn main() {
    input! {
        n: usize,
        p: [usize; n], // p values are 1..=N
    }

    if n < 3 {
        println!("0");
        return;
    }

    // A[i]: count of j < i such that P[j] < P[i]
    let mut bit = FenwickTree::new(n);
    let mut a = vec![0; n];
    for i in 0..n {
        a[i] = bit.sum(p[i] - 1);
        bit.add(p[i], 1);
    }

    // B[i]: count of j > i such that P[j] < P[i]
    // Re-use bit, clear it or new one. New one is easier.
    let mut bit2 = FenwickTree::new(n);
    let mut b = vec![0; n];
    for i in (0..n).rev() {
        b[i] = bit2.sum(p[i] - 1);
        bit2.add(p[i], 1);
    }

    // Precompute powers of 2 and inverse powers of 2
    let mut pow2 = vec![1; n + 1];
    let mut inv2 = vec![1; n + 1];
    let inv2_val = (MOD + 1) / 2;
    for i in 1..=n {
        pow2[i] = pow2[i - 1] * 2 % MOD;
        inv2[i] = inv2[i - 1] * inv2_val % MOD;
    }

    let mut ans = 0;

    // Term 1: L == R (Sequence length 3 logic pivot)
    // Actually part of the general logic?
    // Logic: sum_{L} A[L]*B[L] covers subsequences of length 3 exactly?
    // If length 3: idx1 < L < idx3. P[idx1] < P[L] > P[idx3].
    // Start Up (idx1 < L), End Down (L > idx3). Correct.
    // The formula sum_{L<R} ... covers length >= 4.
    // Length 3 implies gap between pivot L and R is... wait, L=R is the pivot.
    // So separate sum is correct.
    for i in 0..n {
        let term = a[i] * b[i] % MOD;
        ans = (ans + term) % MOD;
    }

    // Term 2: L < R. Gap size R - L - 1.
    // Sum_{R} [ B[R] * 2^(R-1) * Sum_{L < R} ( A[L] * 2^(-L) ) ]

    // running_sum stores sum_{L < R} (A[L] * inv2[L])
    let mut running_sum = 0;

    // We iterate R. For L < R, we need A[L].
    // Base case: R=0. No L < 0. running_sum=0.
    // First valid R is 1 (L=0).

    // Initialize running_sum with L=0 before starting R=1?
    // A[0] terms.
    running_sum = (running_sum + a[0] * inv2[0]) % MOD;

    for r in 1..n {
        // Calculate contribution where 'r' is the right pivot
        // gap r - l - 1.
        // contribution = B[r] * sum_{l < r} (A[l] * 2^(r - l - 1))
        //              = B[r] * 2^(r-1) * sum_{l < r} (A[l] * 2^(-l))

        let term = b[r] * pow2[r - 1] % MOD * running_sum % MOD;
        ans = (ans + term) % MOD;

        // Update running sum for next step (adding A[r] term)
        let add = a[r] * inv2[r] % MOD;
        running_sum = (running_sum + add) % MOD;
    }

    println!("{}", ans);
}

struct FenwickTree {
    size: usize,
    tree: Vec<i64>,
}

impl FenwickTree {
    fn new(n: usize) -> Self {
        FenwickTree {
            size: n,
            tree: vec![0; n + 1],
        }
    }

    fn add(&mut self, mut idx: usize, val: i64) {
        while idx <= self.size {
            self.tree[idx] += val;
            idx += idx & (!idx + 1); // idx & -idx
        }
    }

    fn sum(&self, mut idx: usize) -> i64 {
        let mut s = 0;
        while idx > 0 {
            s += self.tree[idx];
            idx -= idx & (!idx + 1);
        }
        s
    }
}
