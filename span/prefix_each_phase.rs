use proconio::input;
use std::io::{BufWriter, Write};
// abc449E
// Q. Append the least frequent and smallest number to the end of the sequence.
// A. Prefix cumulative sum for calcuating the operations at each phase
fn main() {
    input! {
        n: usize,
        m: usize,
        a: [usize; n],
        q: usize,
        x: [usize; q]
    }

    let mut c = vec![0usize; m + 1];
    for &val in &a {
        c[val] += 1;
    }

    // Group elements by their initial frequency
    let mut elements_by_count = vec![Vec::new(); n + 2];
    let mut cnt_freq = vec![0usize; n + 2];
    for v in 1..=m {
        cnt_freq[c[v]] += 1;
        elements_by_count[c[v]].push(v);
    }

    let mut pref_cnt = vec![0usize; n + 2];
    let mut pref_sum = vec![0usize; n + 2];
    for i in 0..=n {
        pref_cnt[i + 1] = pref_cnt[i] + cnt_freq[i];
        pref_sum[i + 1] = pref_sum[i] + i * cnt_freq[i];
    }

    // P[K: Frequency]
    let mut p = vec![0usize; n + 2];
    for k in 0..=n {
        // P[K] = K * (Total elements with < K counts) - (Sum of their counts)
        p[k] = k * pref_cnt[k] - pref_sum[k];
    }

    let mut ans = vec![0usize; q];
    // queries_by_phase[K: Frequency] = List of (query_index, r) for queries that will be answered in phase K
    let mut queries_by_phase = vec![Vec::new(); n + 2];

    for i in 0..q {
        let xi = x[i];
        if xi <= n {
            ans[i] = a[xi - 1];
        } else {
            let yi = xi - n;
            if yi > p[n] {
                let rem = (yi - p[n] - 1) % m;
                ans[i] = rem + 1;
            } else {
                let k = p.partition_point(|&val| val < yi);
                let r = yi - p[k - 1];
                queries_by_phase[k].push((i, r));
            }
        }
    }

    let mut bit = Fenwick::new(m);

    for k in 1..=n {
        for &v in &elements_by_count[k - 1] {
            bit.add(v, 1);
        }

        for &(q_idx, r) in &queries_by_phase[k] {
            ans[q_idx] = bit.kth(r);
        }
    }

    let stdout = std::io::stdout();
    let mut out = BufWriter::new(stdout.lock());
    for res in ans {
        writeln!(out, "{}", res).unwrap();
    }
}

struct Fenwick {
    tree: Vec<usize>,
}
impl Fenwick {
    fn new(size: usize) -> Self {
        Self {
            tree: vec![0; size + 1],
        }
    }
    fn add(&mut self, mut i: usize, delta: usize) {
        while i < self.tree.len() {
            self.tree[i] += delta;
            i += i & i.wrapping_neg();
        }
    }
    fn kth(&self, mut k: usize) -> usize {
        let mut idx = 0;
        let mut bit_mask = self.tree.len().next_power_of_two();
        if bit_mask > self.tree.len() {
            bit_mask >>= 1;
        }

        while bit_mask > 0 {
            let next_idx = idx + bit_mask;
            if next_idx < self.tree.len() && self.tree[next_idx] < k {
                idx = next_idx;
                k -= self.tree[idx];
            }
            bit_mask >>= 1;
        }
        idx + 1
    }
}
