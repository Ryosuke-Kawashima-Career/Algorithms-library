/// A compact BitVector supporting O(1) rank queries.
#[derive(Clone, Debug)]
pub struct BitVector {
    blocks: Vec<u64>,
    prefix_ones: Vec<u32>,
}

impl BitVector {
    pub fn new(bits: &[bool]) -> Self {
        let n = bits.len();
        let num_blocks = (n + 63) / 64;
        let mut blocks = vec![0u64; num_blocks];
        let mut prefix_ones = Vec::with_capacity(num_blocks + 1);
        prefix_ones.push(0);

        for (i, &b) in bits.iter().enumerate() {
            if b {
                blocks[i / 64] |= 1u64 << (i % 64);
            }
        }
        let mut acc = 0;
        for &blk in &blocks {
            acc += blk.count_ones();
            prefix_ones.push(acc);
        }
        Self { blocks, prefix_ones }
    }

    /// Returns the bit at index `i`.
    #[inline]
    pub fn get(&self, i: usize) -> bool {
        (self.blocks[i / 64] >> (i % 64)) & 1 == 1
    }

    /// Returns the number of 1s in the half-open interval [0, i).
    #[inline]
    pub fn rank1(&self, i: usize) -> usize {
        let block_idx = i / 64;
        let bit_idx = i % 64;
        let mut ans = self.prefix_ones[block_idx] as usize;
        if bit_idx > 0 {
            let mask = (1u64 << bit_idx) - 1;
            ans += (self.blocks[block_idx] & mask).count_ones() as usize;
        }
        ans
    }

    /// Returns the number of 0s in the half-open interval [0, i).
    #[inline]
    pub fn rank0(&self, i: usize) -> usize {
        i - self.rank1(i)
    }
}

pub struct WaveletMatrix {
    levels: Vec<BitVector>,
    zeros: Vec<usize>,
    bit_len: usize,
}

impl WaveletMatrix {
    /// Builds a Wavelet Matrix for values in [0, 2^bit_len) in O(N * bit_len) time.
    pub fn new(mut data: Vec<u64>, bit_len: usize) -> Self {
        let n = data.len();
        let mut levels = Vec::with_capacity(bit_len);
        let mut zeros = Vec::with_capacity(bit_len);

        for d in 0..bit_len {
            let shift = bit_len - 1 - d;
            let bits: Vec<bool> = data.iter().map(|&x| ((x >> shift) & 1) == 1).collect();
            let bv = BitVector::new(&bits);

            let mut left = Vec::with_capacity(n);
            let mut right = Vec::with_capacity(n);
            for (i, &val) in data.iter().enumerate() {
                if bits[i] {
                    right.push(val);
                } else {
                    left.push(val);
                }
            }
            zeros.push(left.len());
            left.extend(right);
            data = left;
            levels.push(bv);
        }

        Self { levels, zeros, bit_len }
    }

    /// Reconstructs A[i] in O(bit_len) time.
    pub fn access(&self, mut i: usize) -> u64 {
        let mut res = 0u64;
        for d in 0..self.bit_len {
            let bv = &self.levels[d];
            if bv.get(i) {
                res |= 1u64 << (self.bit_len - 1 - d);
                i = self.zeros[d] + bv.rank1(i);
            } else {
                i = bv.rank0(i);
            }
        }
        res
    }

    /// Returns the k-th smallest value (0-indexed) in A[l..r) in O(bit_len) time.
    pub fn quantile(&self, mut l: usize, mut r: usize, mut k: usize) -> u64 {
        assert!(k < r - l, "k out of bounds");
        let mut res = 0u64;
        for d in 0..self.bit_len {
            let bv = &self.levels[d];
            let l0 = bv.rank0(l);
            let r0 = bv.rank0(r);
            let count0 = r0 - l0;

            if k < count0 {
                // Target is among the 0-bit elements
                l = l0;
                r = r0;
            } else {
                // Target is among the 1-bit elements
                k -= count0;
                res |= 1u64 << (self.bit_len - 1 - d);
                l = self.zeros[d] + bv.rank1(l);
                r = self.zeros[d] + bv.rank1(r);
            }
        }
        res
    }

    /// Counts elements strictly less than `upper` in A[l..r) in O(bit_len) time.
    pub fn range_freq_less(&self, mut l: usize, mut r: usize, upper: u64) -> usize {
        if upper >= (1u64 << self.bit_len) {
            return r - l;
        }
        let mut count = 0;
        for d in 0..self.bit_len {
            let bv = &self.levels[d];
            let bit = ((upper >> (self.bit_len - 1 - d)) & 1) == 1;
            let l0 = bv.rank0(l);
            let r0 = bv.rank0(r);

            if bit {
                // All 0-bit elements at this level are strictly smaller than `upper`
                count += r0 - l0;
                l = self.zeros[d] + bv.rank1(l);
                r = self.zeros[d] + bv.rank1(r);
            } else {
                l = l0;
                r = r0;
            }
        }
        count
    }

    /// Counts elements in [lower, upper) within subarray A[l..r) in O(bit_len) time.
    pub fn range_freq(&self, l: usize, r: usize, lower: u64, upper: u64) -> usize {
        if lower >= upper {
            return 0;
        }
        self.range_freq_less(l, r, upper) - self.range_freq_less(l, r, lower)
    }
}

fn main() {
    let a = vec![5, 2, 6, 1, 5, 0, 7, 2];
    let wm = WaveletMatrix::new(a.clone(), 3);

    // 1. Access check
    for i in 0..a.len() {
        assert_eq!(wm.access(i), a[i]);
    }

    // 2. Quantile check on A[1..7) = [2, 6, 1, 5, 0, 7] -> sorted: [0, 1, 2, 5, 6, 7]
    assert_eq!(wm.quantile(1, 7, 2), 2); // 0-indexed 2nd smallest is 2
    assert_eq!(wm.quantile(1, 7, 4), 6); // 0-indexed 4th smallest is 6

    // 3. Range frequency check on A[1..7): values in [2, 6) are {2, 5} -> count = 2
    assert_eq!(wm.range_freq(1, 7, 2, 6), 2);

    println!("All checks passed!");
}
