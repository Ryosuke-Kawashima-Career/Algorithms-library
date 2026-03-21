use proconio::input;
// 典型29
// 普通のセグメント木: 一点を更新する。
// 遅延セグメント木: 区間の更新に使用する。
fn main() {
    input!{w: usize, n: usize}
    // ノード数を入力する。
    let max_func = |x: i64, y: i64| -> i64 {
        if x <= y {
            return y;
        }
        return x;
    };
    let mut seg = LazySegmentTree::new(w, 0, max_func);
    for _ in 0..n {
        input!{l: usize, r: usize}
        // 0indexed
        let (l, r) = (l-1, r-1);
        // [l r)の範囲を計算する。
        let max_val: i64 = seg.get_range(l, r+1);
        seg.replace_range(l, r+1, max_val+1);
        println!("{}", max_val+1);
    }
}

#[derive(Debug)]
pub struct LazySegmentTree<T, F> 
where
    T: Copy + Eq + std::fmt::Debug,
    F: Fn(T, T) -> T,
{
    base: usize,
    n: usize,
    e: T,
    // tree[idx].0: actual data, tree[idx].1: Lazy
    tree: Vec<(T, T)>,
    segfunc: F,
}

impl<T, F> LazySegmentTree<T, F> 
where
    T: Copy + Eq + std::fmt::Debug,
    F: Fn(T, T) -> T + Copy,
{
    pub fn new(n: usize, e: T, segfunc: F) -> Self {
        let base = 1 << (64 - n.leading_zeros());
        Self {
            base,
            n,
            e,
            tree: vec![(e, e); base << 1],
            segfunc,
        }
    }

    pub fn from_slice(data: &Vec<T>, e: T, segfunc: F) -> Self {
        let n: usize = data.len();
        let base = 1 << (64 - n.leading_zeros());
        let mut tree = vec![(e, e); base << 1];
        for i in 0..n {
            tree[base + i].0 = data[i];
        }
        Self {
            base,
            n,
            e,
            tree,
            segfunc,
        }
    }

    pub fn propagate(&mut self, idx: usize) {
        if self.tree[idx].1 != self.e {
            self.tree[idx].0 = (self.segfunc)(self.tree[idx].0, self.tree[idx].1);
            if idx < self.base {
                self.tree[idx << 1].1 = self.tree[idx].1;
                self.tree[(idx << 1) + 1].1 = self.tree[idx].1;
            }
            self.tree[idx].1 = self.e;
        }
    }

    pub fn replace_range(&mut self, l: usize, r: usize, value: T) {
        fn _replace_range<T, F>(
            lmrq: &mut LazySegmentTree<T, F>,
            a: usize,
            b: usize,
            l: usize,
            r: usize,
            idx: usize,
            value: T,
        ) 
        where
            T: Copy + Eq + std::fmt::Debug,
            F: Fn(T, T) -> T + Copy,
        {
            if a >= r || b <= l {
                return;
            }
            lmrq.propagate(idx);
            lmrq.tree[idx].0 = (lmrq.segfunc)(lmrq.tree[idx].0, value);
            if a >= l && b <= r {
                if idx < lmrq.base {
                    lmrq.tree[idx << 1].1 = (lmrq.segfunc)(lmrq.tree[idx << 1].1, value);
                    lmrq.tree[(idx << 1) + 1].1 = (lmrq.segfunc)(lmrq.tree[(idx << 1) + 1].1, value);
                }
                return;
            }
            let m = (a + b) >> 1;
            _replace_range(lmrq, a, m, l, r, idx << 1, value);
            _replace_range(lmrq, m, b, l, r, (idx << 1) + 1, value);
        }
        _replace_range(self, 0, self.base, l, r, 1, value);
    }

    pub fn op_range(&mut self, l: usize, r: usize, op_func: F, value: T) {
        fn _op_range<T, F>(
            lmrq: &mut LazySegmentTree<T, F>,
            a: usize,
            b: usize,
            l: usize,
            r: usize,
            op_func: F,
            idx: usize,
            value: T,
        ) 
        where
            T: Copy + Eq + std::fmt::Debug,
            F: Fn(T, T) -> T + Copy,
        {
            if a >= r || b <= l {
                return;
            }
            lmrq.propagate(idx);
            lmrq.tree[idx].0 = (op_func)(lmrq.tree[idx].0, value);
            if a >= l && b <= r {
                if idx < lmrq.base {
                    lmrq.tree[idx << 1].1 = (op_func)(lmrq.tree[idx << 1].1, value);
                    lmrq.tree[(idx << 1) + 1].1 = (op_func)(lmrq.tree[(idx << 1) + 1].1, value);
                }
                return;
            }
            let m = (a + b) >> 1;
            _op_range(lmrq, a, m, l, r, op_func, idx << 1, value);
            _op_range(lmrq, m, b, l, r, op_func, (idx << 1) + 1, value);
        }
        _op_range(self, 0, self.base, l, r, op_func, 1, value);
    }

    pub fn get_range(&mut self, l: usize, r: usize) -> T {
        fn _get_range<T, F>(
            lrmq: &mut LazySegmentTree<T, F>,
            a: usize,
            b: usize,
            l: usize,
            r: usize,
            idx: usize,
        ) -> T 
        where
            T: Copy + Eq + std::fmt::Debug,
            F: Fn(T, T) -> T + Copy,
        {
            if a >= r || b <= l {
                return lrmq.e;
            }
            lrmq.propagate(idx);
            if a >= l && b <= r {
                return lrmq.tree[idx].0;
            }
            let m = (a + b) >> 1;
            let left = _get_range(lrmq, a, m, l, r, idx << 1);
            let right = _get_range(lrmq, m, b, l, r, (idx << 1) + 1);
            return (lrmq.segfunc)(left, right);
        }
        _get_range(self, 0, self.base, l, r, 1)
    }

    pub fn get(&mut self, idx: usize) -> T {
        self.get_range(idx, idx+1)
    }

    pub fn set(&mut self, idx: usize, value: T) {
        self.replace_range(idx, idx+1, value);
    }
}