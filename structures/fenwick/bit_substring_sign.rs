use proconio::{input, marker::Chars};
// abc441e
// Q. Count the number of Substrings comprised of 'A', 'B', 'C' such that 'A' is more than 'B''C'
// A. Equation Transformation
/* S の i 文字目から j 文字目までを取って得られる部分文字列が条件を満たすことは、Aj−Ai−1>Bj−Bi−1と言い換えることができます。これは式変形をすることで, Aj−Bj>Ai−1−Bi−1となります。 */
fn main() {
    input! {n: usize, s: Chars}
    // A: +1 B: -1 C: 0
    let mut fenwick: FenwickTree = FenwickTree::new(2 * n + 1);
    let mut substrings: usize = 0;
    let mut sign: i64 = 0;
    let sign_to_index = |sign: i64| -> usize { (n as i64 + sign) as usize };
    // Do not forget Initial Value
    fenwick.add(sign_to_index(0), 1);
    for i in 0..n {
        sign += match s[i] {
            'A' => 1,
            'B' => -1,
            _ => 0,
        };
        let index: usize = sign_to_index(sign);
        // fenwick uses 1 indexed & Inclusiveness
        substrings += fenwick.sum(index - 1) as usize;
        fenwick.add(index, 1);
    }
    println!("{}", substrings);
}

struct FenwickTree {
    tree: Vec<i64>,
}

impl FenwickTree {
    #[inline]
    fn lsb(n: usize) -> usize {
        /* Returns the least significant bit of n */
        n & n.wrapping_neg()
    }

    #[inline]
    fn next_power_of_two(n: usize) -> usize {
        /* Returns the smallest power of two greater than or equal to n */
        let mut power: usize = 1;
        while power < n {
            power *= 2;
        }
        power
    }

    fn new(n: usize) -> Self {
        Self {
            tree: vec![0; Self::next_power_of_two(n) + 1],
        }
    }

    fn sum(&self, index: usize) -> i64 {
        let mut result: i64 = 0;
        let mut i: usize = index + 1;
        while i > 0 {
            result += self.tree[i];
            i -= Self::lsb(i);
        }
        return result;
    }

    fn add(&mut self, index: usize, val: i64) {
        let mut i: usize = index + 1;
        while i < self.tree.len() {
            self.tree[i] += val;
            i += Self::lsb(i);
        }
    }
}
