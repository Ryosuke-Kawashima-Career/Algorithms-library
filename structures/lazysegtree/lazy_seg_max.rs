use proconio::{input, marker::Usize1};
// 典型29
// 普通のセグメント木: 一点を更新する。
// 遅延セグメント木: 区間の更新に使用する。
fn main() {
    input!{w: usize, n: usize}
    let mut seg =  LazySegTree::new(w, 0, |x, y| x.max(y));
    for _ in 0..n {
        input!{l: Usize1, r: Usize1}
        // [l r)の範囲を計算する。
        /* seg.custom_update(l..=r, max_val+1, |x, y| y); y=val */
        let max_val: i64 = seg.get(l..=r);
        seg.update(l..=r, max_val+1);
        println!("{}", max_val+1);
    }
}

#[derive(Debug)]
struct LazySegTree<M, F>
where
    F: Fn(M, M) -> M,
    M: Copy + Eq,
{
    n: usize,
    size: usize,
    data: Vec<M>,
    lazy: Vec<M>,

    e: M,
    op: F,
}

impl<M, F> LazySegTree<M, F>
where
    F: Fn(M, M) -> M,
    M: Copy + Eq,
{
    fn new(n: usize, e: M, op: F) -> Self {
        let mut size = 1;
        while size < n {
            size *= 2;
        }
        assert!(op(e, e) == e);
        Self {
            n: n,
            size: size,
            data: vec![e; size * 2],
            lazy: vec![e; size * 2],
            e: e,
            op: op,
        }
    }

    pub fn from_slice(a: &[M], e: M, op: F) -> Self {
        let n: usize = a.len();
        let mut seg = Self::new(n, e, op);
        for i in 0..n {
            seg.update_at(i, a[i]);
        }
        seg
    }

    fn _eval(&mut self, i: usize) {
        if self.lazy[i] == self.e {
            return;
        }
        if i < self.size - 1 {
            self.lazy[i * 2 + 1] = (self.op)(self.lazy[i * 2 + 1], self.lazy[i]);
            self.lazy[i * 2 + 2] = (self.op)(self.lazy[i * 2 + 2], self.lazy[i]);
        }
        self.data[i] = (self.op)(self.data[i], self.lazy[i]);
        self.lazy[i] = self.e;
    }

    fn _update(&mut self, a: usize, b: usize, val: M, k: usize, l: usize, r: usize) {
        self._eval(k);
        if a <= l && r <= b {
            // self.op ?
            self.lazy[k] = (self.op)(self.lazy[k], val);
            self._eval(k);
        } else if a < r && l < b {
            self._update(a, b, val, k * 2 + 1, l, (l + r) / 2);
            self._update(a, b, val, k * 2 + 2, (l + r) / 2, r);
            self.data[k] = (self.op)(self.data[k * 2 + 1], self.data[k * 2 + 2]);
        }
    }
    fn _custom_update<G>(&mut self, a: usize, b: usize, val: M, k: usize, l: usize, r: usize, f: G) 
    where
        G: Fn(M, M) -> M + Copy,
    {
        self._eval(k);
        if a <= l && r <= b {
            self.lazy[k] = (f)(self.lazy[k], val);
            self._eval(k);
        } else if a < r && l < b {
            self._custom_update(a, b, val, k * 2 + 1, l, (l + r) / 2, f);
            self._custom_update(a, b, val, k * 2 + 2, (l + r) / 2, r, f);
            self.data[k] = (self.op)(self.data[k * 2 + 1], self.data[k * 2 + 2]);
        }
    }

    pub fn update<R: std::ops::RangeBounds<usize>>(&mut self, rng: R, val: M) {
        let a = match rng.start_bound() {
            std::ops::Bound::Included(i) => *i,
            std::ops::Bound::Excluded(i) => *i + 1,
            std::ops::Bound::Unbounded => 0,
        };
        let b = match rng.end_bound() {
            std::ops::Bound::Included(i) => *i + 1,
            std::ops::Bound::Excluded(i) => *i,
            std::ops::Bound::Unbounded => 0,
        };
        // 0indexed -> 1indexed
        self._update(a+1, b+1, val, 0, 0, self.size);
    }
    pub fn custom_update<R: std::ops::RangeBounds<usize>, G: Fn(M, M) -> M + Copy>(&mut self, rng: R, val: M, f: G) {
        let a = match rng.start_bound() {
            std::ops::Bound::Included(i) => *i,
            std::ops::Bound::Excluded(i) => *i + 1,
            std::ops::Bound::Unbounded => 0,
        };
        let b = match rng.end_bound() {
            std::ops::Bound::Included(i) => *i + 1,
            std::ops::Bound::Excluded(i) => *i,
            std::ops::Bound::Unbounded => 0,
        };

        // 0indexed -> 1indexed
        self._custom_update(a+1, b+1, val, 0, 0, self.size, f);
    }

    pub fn update_at(&mut self, i: usize, val: M) {
        self.update(i..(i + 1), val);
    }
    pub fn custom_update_at<G>(&mut self, i: usize, val: M, f: G) 
    where
        G: Fn(M, M) -> M + Copy,
    {
        self.custom_update(i..(i + 1), val, f);
    }

    fn _get(&mut self, a: usize, b: usize, k: usize, l: usize, r: usize) -> M {
        self._eval(k);
        if r <= a || b <= l {
            self.e
        } else if a <= l && r <= b {
            self.data[k]
        } else {
            let v1 = self._get(a, b, k * 2 + 1, l, (l + r) / 2);
            let v2 = self._get(a, b, k * 2 + 2, (l + r) / 2, r);
            return (self.op)(v1, v2);
        }
    }

    pub fn get<R: std::ops::RangeBounds<usize>>(&mut self, rng: R) -> M {
        let a = match rng.start_bound() {
            std::ops::Bound::Included(i) => *i,
            std::ops::Bound::Excluded(i) => *i + 1,
            std::ops::Bound::Unbounded => 0,
        };
        let b = match rng.end_bound() {
            std::ops::Bound::Included(i) => *i + 1,
            std::ops::Bound::Excluded(i) => *i,
            std::ops::Bound::Unbounded => 0,
        };
        // 0indexed -> 1indexed
        self._get(a+1, b+1, 0, 0, self.size)
    }

    pub fn get_at(&mut self, i: usize) -> M {
        self.get(i..(i + 1))
    }
}