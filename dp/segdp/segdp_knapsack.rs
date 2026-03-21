use proconio::input;
// 典型37
// -INF: impossible
const INF: i64 = 1 << 60;
// segtreeでdpの区間計算を高速化
fn main() {
    input!{w: usize, n: usize, lrv: [(usize, usize, i64); n]}
    fn max_func(a: i64, b: i64) -> i64 {
        if a <= b {
            return b;
        }
        return a;
    }
    let mut dp = vec![SegTree::new(&vec![-INF; w+1], max_func, -INF); n+1];
    dp[0].set(0, 0);
    for i in 1..=n {
        for j in 0..=w {
            // do nothing
            let prev_val: i64 = dp[i-1].get(j);
            if dp[i].get(j) < prev_val {
                dp[i].set(j, prev_val);
            }

            // index: [l r)
            let l: i64 = 0.max(j as i64 - lrv[i-1].1 as i64);
            let r: i64 = (w as i64).min(j as i64 - lrv[i-1].0 as i64);
            if l > r {
                continue;
            }
            if r < 0 {
                continue;
            }
            let (l, r) = (l as usize, r as usize);
            
            let max_val: i64 = dp[i-1].prod(l, r+1);
            if max_val != -INF && max_val + lrv[i-1].2 > dp[i].get(j) {
                dp[i].set(j, max_val + lrv[i-1].2);
            }
        }
    }
    if dp[n].get(w) == -INF {
        println!("-1");
    } else {
        println!("{}", dp[n].get(w));
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

#[derive(Debug, Clone)]
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