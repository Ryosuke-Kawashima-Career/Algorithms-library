use proconio::input;
const INF: i64 = 1 << 60;
// セグメント木は区間内[l r)の最大値を返す。
// 区間クエリが得意
fn main() {
    input!{n: usize, q: usize}
    // max: e = -1 (-1を更新していく.)
    fn max_func(a: i64, b: i64) -> i64 {
        if a <= b {
            return b;
        }
        return a;
    }
    let mut segtree = SegTree::new(&vec![0; n], max_func, -1);
    // min: e = inf (infを更新していく.)
    fn min_func(a: i64, b: i64) -> i64 {
        if a <= b {
            return a;
        }
        return b;
    }
    let mut segtree = SegTree::new(&vec![INF; n], min_func, INF);
    // sum: e = 0
    fn sum_func(a: i64, b: i64) -> i64 {
        a + b
    }
    let mut segtree = SegTree::new(&vec![0; n], sum_func, 0);
    for _ in 0..q {
        input!{query: usize}
        if query == 1 {
            input!{pos: usize, x: i64}
            segtree.set(pos-1, x);
        } else {
            input!{l: usize, r: usize}
            let (l, r) = (l-1, r-1);
            // [l r)
            let sum_val: i64 = segtree.prod(l, r);
            println!("{}", sum_val);
        }
    }
}

/**Segment Tree
*
* ・つかい方
* (配列lis, 関数segfunc, 単位元e を用意して、)
* let st = SegTree::new(lis:&Vec<T>, segfunc:F, e:T);
*
* st.set(idx:usize, value:T): 0-indexed 配列の値を1つだけ変更する
* let val = st.get(idx:usize): 0-indexed 配列の値を返す
* let val = st.prod(l:usize, r:usize): 半開区間[l,r) で関数をつかったときの値を返す
*/

#[derive(Debug)]
pub struct SegTree<T, F>
where
    T: Copy + Eq + std::fmt::Debug,
    F: Fn(T, T) -> T,
{
    data: Vec<T>,
    segfunc: F,
    e: T,
    size: usize,
    height: usize,
}
impl<T, F> SegTree<T, F>
where
    T: Copy + Eq + std::fmt::Debug,
    F: Fn(T, T) -> T,
{
    pub fn new(lis: &Vec<T>, segfunc: F, e: T) -> Self {
        let n0 = lis.len();
        let height = format!("{:b}", n0 - 1).to_string().len();
        let size = 1 << height;
        let mut data = vec![e; size << 1];
        for i in 0..n0 {
            data[size + i] = lis[i];
        }
        for k in (1..=size - 1).rev() {
            let k2 = k << 1;
            data[k] = segfunc(data[k2], data[k2 + 1]);
        }
        Self {
            data,
            segfunc,
            e,
            size,
            height,
        }
    }

    pub fn set(&mut self, idx: usize, value: T) {
        let ni = idx + self.size;
        self.data[ni] = value;
        for k in 1..=self.height {
            let k = ni >> k;
            let k2 = k << 1;
            self.data[k] = (self.segfunc)(self.data[k2], self.data[k2 + 1]);
        }
    }

    pub fn get(&self, idx: usize) -> T {
        self.data[idx + self.size]
    }

    pub fn prod(&self, mut l: usize, mut r: usize) -> T {
        let mut sml = self.e;
        let mut smr = self.e;
        l += self.size;
        r += self.size;
        while l < r {
            if l & 1 == 1 {
                sml = (self.segfunc)(sml, self.data[l]);
                l += 1;
            }
            if r & 1 == 1 {
                smr = (self.segfunc)(smr, self.data[r - 1]);
                r -= 1;
            }
            l >>= 1;
            r >>= 1;
        }
        (self.segfunc)(sml, smr)
    }
    pub fn all_prod(&self) -> T {
        self.data[1]
    }
}