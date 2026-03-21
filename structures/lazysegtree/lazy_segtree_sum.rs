use proconio::input;
// 普通のセグメント木: 一点を更新する。
// 遅延セグメント木: 区間の更新に使用する。
// abc340e
// 区間加算クエリ: segtree, BIT(fenwick)
fn main() {
    input!{n: usize, m: usize, a: [i64; n], b: [usize; m]}
    // e: 0, func: add
    let mut lst = LazySegmentTree::<i64, A>::build(&a);

    for i in 0..m {
        let hand: i64 = lst.get_at(b[i]);
        lst.set_at(b[i], 0);
        // 1: c <=> b[i]+1から配り始める
        let index: usize = (b[i] + 1) % n;
        let add_num: i64 = hand / n as i64;
        let plus_index: usize = hand as usize % n;
        lst.custom_update(0, n, A(add_num), |x, y| x.0 + y);
        let remain_index: usize = n - index;
        if plus_index <= remain_index {
            lst.custom_update(index, index + plus_index, A(1), |x, y| x.0 + y);
        } else {
            lst.custom_update(index, n, A(1), |x, y| x.0 + y);
            lst.custom_update(0, plus_index - remain_index, A(1), |x, y| x.0 + y);
        }
    }

    for i in 0..n {
        print!("{} ", lst.get_at(i));
    }
}

impl Monoid for i64 {
    fn e() -> Self {
        0
    }
    // sum: *self + *rhs
    fn op(&self,rhs: &Self) -> Self {
        *self + *rhs
    }
}
#[derive(Clone, Copy)]
struct A(i64);
impl Monoid for A {
    fn op(&self, rhs: &Self) -> Self {
        A(self.0 + rhs.0)
    }
    fn e() -> Self {
        A(0)
    }
}

#[allow(unused)]
pub trait Effect<T> {
    fn eval(&self, x: &T) -> T;
}

// how to update
impl Effect<i64> for A {
    fn eval(&self, x: &i64) -> i64 {
        self.0 + x
    }
}

#[allow(unused)]
pub trait Monoid: Clone + Copy {
    fn e() -> Self;
    fn op(&self, rhs: &Self) -> Self;
}

#[allow(unused)]
pub struct LazySegmentTree<T, E> {
    n: usize,
    size: usize,
    data: Vec<(T, E)>,    
}

#[allow(unused)]
impl<T, E> LazySegmentTree<T, E>
where 
    T: Monoid,
    E: Monoid + Effect<T>,
{
    fn new(n: usize) -> Self {
        let size = n.next_power_of_two();
        Self { n: n, size: size, data: vec![(T::e(), E::e()); 2 * size] }
    }
    fn build(a: &Vec<T>) -> Self {
        let size = a.len().next_power_of_two();
        let mut data = vec![(T::e(), E::e()); 2 * size];
        for i in 0..a.len() {
            data[size + i].0 = a[i];
        }
        let mut seg = Self {
            n: a.len(),
            size: size,
            data: data,
        };
        for i in (1..size).rev() {
            seg.save_at(i);
        }
        seg
    }

    // data_x に遅延fを適用して 子ノードに渡す遅延を計算し直す。
    fn apply_at(&mut self, x: usize, f: &E) {
        let po = &mut self.data[x];
        *po = (f.eval(&po.0), po.1.op(f));
    }
    // func is the same with eval()
    fn custom_apply_at<F>(&mut self, x: usize, f: E, func: F) 
    where
        F: Fn(E, T) -> T + Copy,
    {
        let po = &mut self.data[x];
        *po = (func(f, po.0), po.1.op(&f));
    }

    // data_x の遅延を取り出して、それを子に渡して、子にapply_atを適用する。
    fn propagate_at(&mut self, x: usize) {
        let f = std::mem::replace(&mut self.data[x].1, E::e());
        self.apply_at(2 * x, &f);
        self.apply_at(2 * x + 1, &f);
    }

    // a_x を計算して、data.0に入れる。遅延は頭に入れない。
    fn save_at(&mut self, x: usize) {
        self.data[x].0 = self.data[2 * x].0.op(&self.data[2 * x + 1].0);
    }

    // [l, r) に関して遅延を評価していく。
    fn propagate(&mut self, l: usize, r: usize) {
        let l = l + self.size;
        let r = r + self.size;
        for i in (1..=self.size.trailing_zeros() as usize).rev() {
            if (l >> i) << i != l {
                self.propagate_at(l >> i);
            }
            if (r >> i) << i != r {
                self.propagate_at((r - 1) >> i);
            }
        }
    }

    // [l, r) の遅延は評価に入れずに計算していく。
    fn save(&mut self, l: usize, r: usize) {
        let l = l + self.size;
        let r = r + self.size;
        for i in 1..=self.size.trailing_zeros() as usize {
            if (l >> i) << i != l {
                self.save_at(l >> i);
            }
            if (r >> i) << i != r {
                self.save_at((r - 1) >> i);
            }
        }
    }

    // [l, r) に fの更新をしていく。
    pub fn update(&mut self, l: usize, r: usize, f: E) {
        if l == r {
            return;
        }
        self.propagate(l, r);
        let mut x = l + self.size;
        let mut y = r + self.size;
        while x < y {
            if x & 1 == 1 {
                self.apply_at(x, &f);
                x += 1;
            }
            if y & 1 == 1 {
                y -= 1;
                self.apply_at(y, &f);
            }
            x >>= 1;
            y >>= 1;
        }
        self.save(l, r);
    }
    pub fn custom_update<F>(&mut self, l: usize, r: usize, f: E, func: F) 
    where
        F: Fn(E, T) -> T + Copy,
    {
        if l == r {
            return;
        }
        self.propagate(l, r);
        let mut x = l + self.size;
        let mut y = r + self.size;
        while x < y {
            if x & 1 == 1 {
                self.custom_apply_at(x, f, func);
                x += 1;
            }
            if y & 1 == 1 {
                y -= 1;
                self.custom_apply_at(y, f, func);
            }
            x >>= 1;
            y >>= 1;
        }
        self.save(l, r);
    }

    // [l, r) op() を求める。
    pub fn get(&mut self, l: usize, r: usize) -> T {
        if l == r {
            return T::e();
        }
        self.propagate(l, r);
        let mut x = l + self.size;
        let mut y = r + self.size;
        let mut p = T::e();
        let mut q = T::e();
        while x < y {
            if x & 1 == 1 {
                p = p.op(&self.data[x].0);
                x += 1;
            }
            if y & 1 == 1 {
                y -= 1;
                q = self.data[y].0.op(&q);
            }
            x >>= 1;
            y >>= 1;
        }
        p.op(&q)
    }

    // a_x に v をセットする。
    pub fn set_at(&mut self, x: usize, v: T) {
        let y = x + self.size;
        for i in (1..=self.size.trailing_zeros() as usize).rev() {
            self.propagate_at(y >> i);
        }
        self.data[y].0 = v;
        for i in 1..=self.size.trailing_zeros() as usize {
            self.save_at(y >> i);
        }
    }

    // a_x の値を取得する。
    pub fn get_at(&mut self, x: usize) -> T {
        let y = x + self.size;
        for i in (1..=self.size.trailing_zeros() as usize).rev() {
            self.propagate_at(y >> i);
        }
        self.data[y].0.clone()
    }
}