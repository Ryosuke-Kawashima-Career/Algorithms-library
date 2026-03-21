use proconio::input;
// 区間加算クエリ: segtree, BIT(Fenwick)
fn main() {
    let a: Vec<i64> = vec![1, 2, 3, 4, 5];
    let mut fenwick = Fenwick::from_slice(&a);
    let index: usize = fenwick.lower_bound(0);
    println!("{}", index);
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

    pub fn lower_bound(&mut self, k: i64) -> usize {
        let mut ng = -1isize;
        let mut ok = self.n as isize - 1;
        while ok - ng > 1 {
            let mid = (ok + ng) / 2;
            if self.sum(mid as usize) >= k {
                ok = mid;
            } else {
                ng = mid;
            }
        }
        ok as usize
    }
    pub fn upper_bound(&mut self, k: i64) -> usize {
        let mut ng = -1isize;
        let mut ok = self.n as isize - 1;
        while ok - ng > 1 {
            let mid = (ok + ng) / 2;
            if self.sum(mid as usize) > k {
                ok = mid;
            } else {
                ng = mid;
            }
        }
        ok as usize
    }
}