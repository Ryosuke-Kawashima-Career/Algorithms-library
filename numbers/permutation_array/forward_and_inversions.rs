fn forward_and_inversions(p: &[usize]) {
    /* P is a permutation of 1..=N */
    let n = p.len();
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
}
