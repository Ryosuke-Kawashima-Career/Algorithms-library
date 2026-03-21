use proconio::input;
// 典型29
// 普通のセグメント木: 一点を更新する。
// 遅延セグメント木: 区間の更新に使用する。
fn main() {
    input!{w: usize, n: usize}
    // ノード数を入力する。
    let mut seg = LazyRangeMaxQueue::new(w);
    for _ in 0..n {
        input!{l: usize, r: usize}
        // 0indexed
        let (l, r) = (l-1, r-1);
        // [l r)の範囲を計算する。
        let max_val: i64 = seg.get_range_max(l, r+1);
        seg.range_replace(l, r+1, max_val+1);
        println!("{}", max_val+1);
    }
}

// val -> -valとすれば最小値を選べる。
// 負の数が必要な時
// fn new(n: usize) -> Self {
//     let base = 1 << (64 - n.leading_zeros());
//     // init number
//     let inf = 1 << 60;
//     Self {
//         base,
//         tree: vec![(-inf, -inf); base << 1],
//     }
// }
#[derive(Debug, Clone)]
struct LazyRangeMaxQueue {
    base: usize,
    tree: Vec<(i64, i64)>,
}

impl LazyRangeMaxQueue {
    fn new(n: usize) -> Self {
        let base = 1 << (64 - n.leading_zeros());
        Self {
            base,
            tree: vec![(0, 0); base << 1],
        }
    }

    fn propagate(&mut self, idx: usize) {
        if self.tree[idx].1 != 0 {
            self.tree[idx].0 = self.tree[idx].0.max(self.tree[idx].1);
            if idx < self.base {
                self.tree[idx << 1].1 = self.tree[idx].1;
                self.tree[(idx << 1) + 1].1 = self.tree[idx].1;
            }
            self.tree[idx].1 = 0;
        }
    }

    fn range_replace(&mut self, l: usize, r: usize, value: i64) {
        fn _range_replace(
            lmrq: &mut LazyRangeMaxQueue,
            a: usize,
            b: usize,
            l: usize,
            r: usize,
            idx: usize,
            value: i64,
        ) {
            if a >= r || b <= l {
                return;
            }
            lmrq.propagate(idx);
            lmrq.tree[idx].0 = lmrq.tree[idx].0.max(value);
            if a >= l && b <= r {
                if idx < lmrq.base {
                    lmrq.tree[idx << 1].1 = lmrq.tree[idx << 1].1.max(value);
                    lmrq.tree[(idx << 1) + 1].1 = lmrq.tree[(idx << 1) + 1].1.max(value);
                }
                return;
            }
            let m = (a + b) >> 1;
            _range_replace(lmrq, a, m, l, r, idx << 1, value);
            _range_replace(lmrq, m, b, l, r, (idx << 1) + 1, value);
        }
        _range_replace(self, 0, self.base, l, r, 1, value);
    }

    fn get_range_max(&mut self, l: usize, r: usize) -> i64 {
        fn _get_range_max(
            lrmq: &mut LazyRangeMaxQueue,
            a: usize,
            b: usize,
            l: usize,
            r: usize,
            idx: usize,
        ) -> i64 {
            if a >= r || b <= l {
                return 0;
            }
            lrmq.propagate(idx);
            if a >= l && b <= r {
                return lrmq.tree[idx].0;
            }
            let m = (a + b) >> 1;
            let left = _get_range_max(lrmq, a, m, l, r, idx << 1);
            let right = _get_range_max(lrmq, m, b, l, r, (idx << 1) + 1);
            return left.max(right);
        }
        _get_range_max(self, 0, self.base, l, r, 1)
    }
}