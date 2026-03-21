use proconio::input;
// abc340e
// 区間加算クエリ: segtree, BIT(Fenwick)
fn main() {
    input!{n: usize, m: usize, a: [i64; n], b: [usize; m]}
    let mut fenwick = Fenwick::from_slice(&a);

    for i in 0..m {
        let hand: i64 = fenwick.get(b[i]);
        fenwick.set(b[i], 0);
        // 1: c
        let index: usize = (b[i] + 1) % n;
        let add_num: i64 = hand / n as i64;
        let plus_index: usize = hand as usize % n;
        fenwick.add_range(0, n, add_num);
        let remain_index: usize = n - index;
        if plus_index <= remain_index {
            fenwick.add_range(index, index + plus_index, 1);
        } else {
            fenwick.add_range(index, n, 1);
            fenwick.add_range(0, plus_index - remain_index, 1);
        }
    }

    for i in 0..n {
        print!("{} ", fenwick.get(i));
    }
}

/* BIT: 区間和の更新や計算を行う構造体
    初期値は a_1 = a_2 = ... = a_n = 0
    ・add(i,x): a_i += x とする
    ・sum(i): a_1 + a_2 + ... + a_i を計算する
    計算量は全て O(logn)
*/
// impl: 1indexed, interface: 0indexed

struct Fenwick {
    n: usize, sums: Vec<Vec<i64>>
}

impl Fenwick {
    // Least Significant Bit
    #[inline]
    fn lsb(x: usize) -> usize {
        x & x.wrapping_neg()
    }

    pub fn new(length: usize) -> Self {
        Self { n: length+1, sums: vec![vec![0; length+1]; 2] }
    }

    pub fn from_slice(vector: &Vec<i64>) -> Self {
        let length: usize = vector.len();
        let mut res = Fenwick::new(length);
        for i in 0..length {
            res.add(i, vector[i]);
        }

        return res;
    }

    pub fn add_sub(&mut self, phase: usize, i: usize, x: i64) {
        // 0indexed -> 1indexed
        let mut index: usize = i+1;
        while index < self.n {
            self.sums[phase][index] += x;
            index += Self::lsb(index)
        } 
    }

    // add [l r)
    pub fn add_range(&mut self, l: usize, r: usize, x: i64) {
        self.add_sub(0, l, -x * (l as i64 - 1));
        self.add_sub(0, r, x * (r as i64 - 1));
        self.add_sub(1, l, x);
        self.add_sub(1, r, -x);
    }

    pub fn add(&mut self, i: usize, x: i64) {
        self.add_range(i, i+1, x);
    }

    // sum of a[1]~=a[i]
    pub fn sum_sub(&mut self, phase: usize, i: usize) -> i64 {
        let mut res: i64 = 0;
        // 0indexed -> 1indexed
        let mut index: usize = i+1;
        while index > 0 {
            res += self.sums[phase][index];
            index = index.saturating_sub(Self::lsb(index));
        }

        return res;
    }

    // sum of 0..=i
    pub fn sum(&mut self, i: usize) -> i64 {
        return self.sum_sub(0, i) + self.sum_sub(1, i) * (i as i64);
    }

    // sum of [l r)
    pub fn sum_range(&mut self, l: usize, r: usize) -> i64 {
        return self.sum(r-1) - self.sum(l-1);
    }

    // get the value of index
    pub fn get(&mut self, index: usize) -> i64 {
        self.sum_range(index, index + 1)
    }

    // set A[i] = value
    pub fn set(&mut self, index: usize, value: i64) {
        let cur_value: i64 = self.get(index);
        self.add(index, -cur_value);
        self.add(index, value);
    }

    pub fn lower_bound(&self, mut x: i64) -> usize {
        let mut i = 0;
        let mut k = 1;
        while k <= self.n {
            k <<= 1;
        }
        while k > 0 {
            if i + k <= self.n && self.sums[0][i + k - 1] < x {
                x -= self.sums[0][i + k - 1];
                i += k;
            }
            k >>= 1;
        }
        if x > 0 {
            i
        } else {
            0
        }
    }
    pub fn upper_bound(&self, mut x: i64) -> usize {
        let mut i = 0;
        let mut k = 1;
        while k <= self.n {
            k <<= 1;
        }
        while k > 0 {
            if i + k <= self.n && self.sums[0][i + k - 1] <= x {
                x -= self.sums[0][i + k - 1];
                i += k;
            }
            k >>= 1;
        }
        if i < self.n {
            i
        } else {
            self.n
        }
    }

    pub fn binary_left(&mut self, k: i64) -> usize {
        let mut ng = 0;
        let mut ok = self.n;
        while ok - ng > 1 {
            let mid = (ok + ng) / 2;
            if self.sum(mid) >= k {
                ok = mid;
            } else {
                ng = mid;
            }
        }
        ok
    }
    pub fn binary_right(&mut self, k: i64) -> usize {
        let mut ng = 0;
        let mut ok = self.n;
        while ok - ng > 1 {
            let mid = (ok + ng) / 2;
            if self.sum(mid) > k {
                ok = mid;
            } else {
                ng = mid;
            }
        }
        ok
    }
}