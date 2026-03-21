use proconio::{input, marker::Usize1};
// ABC435E
// Q. lr continues to come, paint [l, r] to black. After each query, output how many white cells remain.
// A. クエリ先読みができるとき、動的セグ木を使わずとも座標圧縮 + セグ木でできる場合が多い
fn main() {
    input! {n: usize, q: usize, lr: [(Usize1, Usize1); q]}
    // 1. Coordinate Compression
    let mut xs = Vec::with_capacity(q * 2);
    for &(l, r) in &lr {
        xs.push(l);
        xs.push(r + 1);
    }
    xs.sort();
    xs.dedup();
    // 2. Build Tree on COMPRESSED coordinates
    let m = if xs.is_empty() { 0 } else { xs.len() - 1 };
    let mut initial_data = Vec::with_capacity(m);
    for i in 0..m {
        // Track real length of this compressed interval
        let len = (xs[i + 1] - xs[i]) as i64;
        initial_data.push(Data { val: 0, len });
    }
    let mut lazyseg = LazySegmentTree::build(&initial_data);
    // 3. Process Queries using Binary Search on xs
    let mut answer: Vec<i64> = Vec::new();
    for i in 0..q {
        let (l, r) = lr[i];
        let l_idx = xs.binary_search(&l).unwrap();
        let r_idx = xs.binary_search(&(r + 1)).unwrap();

        lazyseg.update(l_idx, r_idx, A(1));

        let blacks = if m == 0 { 0 } else { lazyseg.get(0, m).val };
        answer.push(n as i64 - blacks);
    }
    for ans in answer {
        println!("{}", ans);
    }
}

#[derive(Clone, Copy, Debug)]
struct Data {
    val: i64,
    len: i64,
}

impl Monoid for Data {
    fn e() -> Self {
        Data { val: 0, len: 0 }
    }
    // merge: sum values and sum lengths
    fn op(&self, rhs: &Self) -> Self {
        Data {
            val: self.val + rhs.val,
            len: self.len + rhs.len,
        }
    }
}

#[derive(Clone, Copy, Debug)]
struct A(i64);

impl Monoid for A {
    fn e() -> Self {
        A(0)
    }
    // update: 1 overrides (Logical OR equivalent)
    fn op(&self, rhs: &Self) -> Self {
        A(self.0 | rhs.0)
    }
}

pub trait Effect<T> {
    fn eval(&self, x: &T) -> T;
}

// how to update
impl Effect<Data> for A {
    fn eval(&self, x: &Data) -> Data {
        if self.0 == 1 {
            // "Turn to 1": The sum becomes equal to the length of the range
            Data {
                val: x.len,
                len: x.len,
            }
        } else {
            // Identity update (0): Do nothing
            *x
        }
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
        Self {
            n: n,
            size: size,
            data: vec![(T::e(), E::e()); 2 * size],
        }
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
