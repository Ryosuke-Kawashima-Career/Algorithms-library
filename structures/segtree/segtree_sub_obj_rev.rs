use proconio::{input, marker::Usize1};
const MAX: usize = 500_000;
// Q. Find the value of 1≤i≤N∑max(l,min(r,Ai)).
// A. Contributions of Ai: 主客転倒(各値の寄与を計算する)
// A. Ai=jを満たすiの個数をCj
fn main() {
    input!{n: usize, q: usize, mut a: [usize; n]}
    // segtree[a[i]] (counts for 0..a[i], counts * 0..a[i] = contribution of 0..a[i])
    let mut segtree = SegTree::new(&vec![(0, 0); MAX+1], seg_func, (0, 0));
    let mut answer: Vec<usize> = Vec::new();
    for i in 0..n {
        add(a[i], &mut segtree);
    }
    for _ in 0..q {
        input!{query_type: usize}
        if query_type == 1 {
            input!{x: Usize1, y: usize}
            delete(a[x], &mut segtree);
            a[x] = y;
            add(a[x], &mut segtree);
        } else {
            input!{l: usize, r: usize}
            let mut ans: usize = 0;
            if l > r {
                ans = l * n;
            } else {
                ans += l * segtree.prod(0, l).0;
                ans +=  segtree.prod(l, r+1).1;
                ans += r * segtree.prod(r+1, MAX).0;
            }
            answer.push(ans);
        }
    }
    for ans in answer {
        println!("{}", ans);
    }
}

fn add<F>(x: usize, segtree: &mut SegTree<(usize, usize), F>)
where
    F: Fn((usize, usize), (usize, usize)) -> (usize, usize),
{
    let cur_val: (usize, usize) = segtree.get(x);
    let (counts, contributions) = cur_val;
    segtree.set(x, (counts+1, contributions+x));
}

fn delete<F>(x: usize, segtree: &mut SegTree<(usize, usize), F>)
where
    F: Fn((usize, usize), (usize, usize)) -> (usize, usize)
{
    let cur_val: (usize, usize) = segtree.get(x);
    let (counts, contributions) = cur_val;
    segtree.set(x, (counts.saturating_sub(1), contributions.saturating_sub(x)));
}

/*
  First element: Sum of C_j
  Second element: Sum of C_j \dot j
*/
fn seg_func(x1: (usize, usize), x2: (usize, usize)) -> (usize, usize) {
    return(x1.0+x2.0, x1.1+x2.1);
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