use proconio::{input, marker::Usize1};
// abc343f
// 区間クエリ: 2番目に大きい値の個数を出力
// Node((一番大きい数, その個数), (二番目に大きい数, その個数))
// Nodeの計算を工夫することで計算する
fn main() {
    input!{n: usize, q: usize, a: [i64; n]}
    let a: Vec<((i64, usize), (i64, usize))> = a.iter().map(|&x| ((x, 1), (0, 0))).collect();
    let mut segtree = SegTree::new(&a, segfunc, ((-1, 0), (-2, 0)));
    let mut answer: Vec<usize> = Vec::new();
    for _ in 0..q {
        input!{query: usize}
        if query == 1 {
            input!{p: Usize1, x: i64}
            segtree.set(p, ((x, 1), (0, 0)));
        } else {
            input!{l: Usize1, r: Usize1}
            let second_max_num: usize = segtree.prod(l, r+1).1.1;
            answer.push(second_max_num);
        }
    }

    for &ans in answer.iter() {
        println!("{}", ans);
    }
}

fn segfunc(
    ((max1, max_num1), (second1, second_num1)): ((i64, usize), (i64, usize)),
    ((max2, max_num2), (second2, second_num2)): ((i64, usize), (i64, usize)),
)  -> ((i64, usize), (i64, usize))  {
    let mut max_second: Vec<i64> = vec![max1, max2, second1, second2];
    let mut max_second_num: Vec<usize> = vec![max_num1, max_num2, second_num1, second_num2];
    let mut max_new: i64 = 0;
    let mut max_new_num: usize = 0;
    let mut second_new: i64 = 0;
    let mut second_new_num: usize = 0;
    // 最大値を求める
    max_new = max_new.max(max1);
    max_new = max_new.max(max2);
    // 二番目に大きい値を求める
    for i in 0..4 {
        if max_second[i] != max_new {
            second_new = second_new.max(max_second[i]);
        }
    }
    // 最大値と二番目に大きい数の個数を求める
    for i in 0..4 {
        if max_second[i] == max_new {
            max_new_num += max_second_num[i];
        }
        if max_second[i] == second_new {
            second_new_num += max_second_num[i];
        }
    }
    return ((max_new, max_new_num), (second_new, second_new_num));
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