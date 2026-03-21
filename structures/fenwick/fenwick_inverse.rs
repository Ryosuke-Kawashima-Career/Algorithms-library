use proconio::{input, marker::Usize1};
// Chokudai SpeedRun 001 - J - Inversion Count
// Q. Inversion Count: 転倒数を計算する
// 転倒数: i < j && Ai > Aj
// 平面走査の問題: 二次元座標をソートやデータ構造を使ってパラメータを一つにして解く
// A. Binary Indexed Tree
fn main() {
    input! {n: usize, a: [Usize1; n]}
    let mut fenwick = BinaryIndexedTree::new(n);
    let mut inversion_count: i64 = 0;
    for i in 0..n {
        // 自分よりも小さいにも関わらず、自分の後ろにいる数の個数を数える
        inversion_count += fenwick.sum_range(a[i], n);
        // 自分の数を追加
        fenwick.add(a[i], 1);
    }
    println!("{}", inversion_count);
}

struct BinaryIndexedTree {
    // indexes are 1-based and assgined from the child to parent.
    // the range is [l r] not [l r)
    data: Vec<i64>,
}

impl BinaryIndexedTree {
    // the position of LSB (least significant bit) representing the size of the range
    #[inline]
    fn lsb(n: usize) -> usize {
        return n & n.wrapping_neg();
    }
    fn new(size: usize) -> Self {
        Self {
            data: vec![0; size + 1],
        }
    }
    // add: add x to a[i] from child to parent
    fn add(&mut self, idx0: usize, x: i64) {
        let idx1: usize = idx0 + 1;
        let n: usize = self.data.len() - 1;
        let mut i: usize = idx1;
        while i <= n {
            self.data[i] += x;
            // from child to parent
            i += Self::lsb(i);
        }
    }
    fn get(&mut self, idx0: usize) -> i64 {
        self.sum_range(idx0, idx0 + 1)
    }
    fn set(&mut self, idx0: usize, x: i64) {
        let current_value = self.get(idx0);
        let diff = x - current_value;
        self.add(idx0, diff);
    }
    fn sum(&self, idx0: usize) -> i64 {
        let idx1: usize = idx0 + 1;
        let mut i: usize = idx1;
        let mut sum_1toi: i64 = 0;
        while i > 0 {
            sum_1toi += self.data[i];
            // from parent to child
            i -= Self::lsb(i);
        }
        return sum_1toi;
    }
    fn sum_range(&self, left0: usize, right0: usize) -> i64 {
        /* Returns sum of [left0, right0) */
        if left0 == 0 {
            return self.sum(right0 - 1);
        }
        return self.sum(right0 - 1) - self.sum(left0 - 1);
    }
}
