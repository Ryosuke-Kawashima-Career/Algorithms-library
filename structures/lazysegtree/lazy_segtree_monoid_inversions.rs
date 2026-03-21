use proconio::{input, marker::Usize1};
// Atcoder Library Practice Contest L - Lazy Segment Tree
/* Question: Xor Inversion Count: 要は[Li,Ri)の区間に対して0と1を反転させる操作と、その区間の反転数を求める操作を行う。
You are given a binary array A=(A1,A2,⋯,AN) of length N.
Process Q queries of the following types. The i-th query is represented by three integers Ti,Li,Ri.
Ti=1: Replace the value of Aj with 1−Ajfor each Li≤j≤Ri.
Ti=2: Calculate the inversion(*) of the array ALi,ALi+1,⋯,ARi． */
fn main() {
    // a has the element of 0 or 1.
    input! {n: usize, q: usize, a: [i64; n], queries: [(usize, Usize1, Usize1); q]}
    let mut lazy_segtree: LazySegmentTree = LazySegmentTree::build(&a);
    for query in 0..q {
        let (t, l, r) = queries[query];
        if t == 1 {
            lazy_segtree.update(l, r + 1, Action { flag: 1 });
        } else {
            let monoid_res: Monoid = lazy_segtree.query(l, r + 1);
            println!("{}", monoid_res.inversions);
        }
    }
}

#[derive(Clone, Copy, Eq, Ord, PartialEq, PartialOrd)]
struct Monoid {
    zeros: i64,
    ones: i64,
    inversions: i64,
}

impl Monoid {
    /* Monoid has an associative operation and identity element. */
    fn op(&self, b: Self) -> Self {
        Self {
            zeros: self.zeros + b.zeros,
            ones: self.ones + b.ones,
            inversions: self.inversions + b.inversions + self.ones * b.zeros,
        }
    }
    fn e() -> Self {
        Self {
            zeros: 0,
            ones: 0,
            inversions: 0,
        }
    }
}

#[derive(Clone, Copy, PartialEq)]
struct Action {
    /* Monoid for lazy propagation */
    flag: i64, // 0: do nothing, 1: flip 0 and 1
}

impl Action {
    fn mapping(&self, b: Monoid) -> Monoid {
        if self.flag == 0 {
            return b;
        } else {
            return Monoid {
                /* flip 0 and 1 = Switching 0 and 1 */
                zeros: b.ones,
                ones: b.zeros,
                inversions: b.zeros * b.ones - b.inversions,
            };
        }
    }
    /* Action has an associative operation and identity element. */
    fn composition(&self, b: Self) -> Self {
        /* composition of actions */
        Action {
            flag: self.flag ^ b.flag,
        }
    }
    fn e() -> Self {
        Action { flag: 0 }
    }
}

struct LazySegmentTree {
    n: usize, // number of leaves (power of two)
    /* node: 1-indexed leaf: 0-indexed */
    data: Vec<Monoid>,
    lazy: Vec<Action>,
}

impl LazySegmentTree {
    #[inline]
    fn next_power_of_two(n: usize) -> usize {
        let mut pow2: usize = 1;
        while pow2 < n {
            pow2 <<= 1;
        }
        return pow2;
    }

    fn new(n: usize) -> Self {
        let n_next_pow2: usize = Self::next_power_of_two(n);
        let data: Vec<Monoid> = vec![Monoid::e(); n_next_pow2 << 1];
        let lazy: Vec<Action> = vec![Action::e(); n_next_pow2 << 1];
        Self {
            n: n_next_pow2,
            data,
            lazy,
        }
    }

    fn build(a: &Vec<i64>) -> Self {
        let n: usize = a.len();
        let n_next_pow2: usize = Self::next_power_of_two(n);
        let mut data: Vec<Monoid> = vec![Monoid::e(); n_next_pow2 << 1];
        let lazy: Vec<Action> = vec![Action::e(); n_next_pow2 << 1];
        /* === Need to change from here === */
        for i in 0..n {
            if a[i] == 0 {
                data[n_next_pow2 + i] = Monoid {
                    zeros: 1,
                    ones: 0,
                    inversions: 0,
                };
            } else {
                data[n_next_pow2 + i] = Monoid {
                    zeros: 0,
                    ones: 1,
                    inversions: 0,
                };
            }
        }
        /* === Need to change to here === */
        for i in (1..n_next_pow2).rev() {
            // |Monoid, Monoid| -> Monoid
            data[i] = data[i << 1].op(data[(i << 1) + 1]);
        }

        Self {
            n: n_next_pow2,
            data,
            lazy,
        }
    }

    fn lazy_evaluate(&mut self, node: usize) {
        /* node: 1-indexed */
        if self.lazy[node] == Action::e() {
            /* no need to update self and the children */
            return;
        }
        if node < self.n {
            // |Action, Action| -> Action
            self.lazy[node << 1] = self.lazy[node].composition(self.lazy[node << 1]);
            self.lazy[(node << 1) + 1] = self.lazy[node].composition(self.lazy[(node << 1) + 1]);
        }
        // apply action to self.data
        self.data[node] = self.lazy[node].mapping(self.data[node]);
        self.lazy[node] = Action::e();
    }

    fn update(&mut self, l0: usize, r0: usize, val: Action) {
        /* l0, r0: 0-indexed [l0, r0) */
        /* child to parent */
        self._update(l0, r0, val, 1, 0, self.n)
    }

    fn _update(
        &mut self,
        l0: usize,
        r0: usize,
        val: Action,
        cur_node: usize,
        cur_left: usize,
        cur_right: usize,
    ) {
        /* l0, r0: 0-indexed [l0, r0) */
        /* if you do not evaluate lazy first, the lazy value will be overwritten */
        self.lazy_evaluate(cur_node);
        if r0 <= cur_left || cur_right <= l0 {
            // out of range
            return;
        } else if l0 <= cur_left && cur_right <= r0 {
            // completely within range
            self.lazy[cur_node] = self.lazy[cur_node].composition(val);
            self.lazy_evaluate(cur_node);
        } else {
            // overlap
            let mid: usize = (cur_left + cur_right) >> 1;
            self._update(l0, r0, val, cur_node << 1, cur_left, mid);
            // operator precedence issue in Rust. The + operator has higher precedence than the << (left shift) operator.
            self._update(l0, r0, val, (cur_node << 1) + 1, mid, cur_right);
            self.data[cur_node] = self.data[cur_node << 1].op(self.data[(cur_node << 1) + 1]);
        }
    }

    fn query(&mut self, l0: usize, r0: usize) -> Monoid {
        /* l0, r0: 0-indexed [l0, r0) */
        /* parent to child */
        self._query(l0, r0, 1, 0, self.n)
    }

    fn _query(
        &mut self,
        l0: usize,
        r0: usize,
        cur_node: usize,
        cur_left: usize,
        cur_right: usize,
    ) -> Monoid {
        self.lazy_evaluate(cur_node);
        /* l0, r0: 0-indexed [l0, r0) */
        if r0 <= cur_left || cur_right <= l0 {
            // out of range
            return Monoid::e();
        } else if l0 <= cur_left && cur_right <= r0 {
            // completely within range
            return self.data[cur_node];
        } else {
            let mid: usize = (cur_left + cur_right) >> 1;
            let val_left: Monoid = self._query(l0, r0, cur_node << 1, cur_left, mid);
            let val_right: Monoid = self._query(l0, r0, (cur_node << 1) + 1, mid, cur_right);
            return val_left.op(val_right);
        }
    }
}
