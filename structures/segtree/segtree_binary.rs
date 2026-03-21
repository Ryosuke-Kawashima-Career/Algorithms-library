use proconio::input;
const INF: i64 = 1 << 60;
// abc320e
fn main(){
    input!{n:usize, m:usize, query:[(i64,i64,i64);m]}

    // min: e = inf (infを更新していく.)
    fn min_func(a: i64, b: i64) -> i64 {
        if a <= b {
            return a;
        }
        return b;
    }
    // それぞれの人が戻る時間
    let mut seg = SegTree::new(&vec![0; n], min_func, INF);
    let mut ans = vec![0; n];

    for &(t, w, s) in &query {
        // 戻る時間が制限時間以内のひとを求める
        let i = seg.max_right(0, |a| a > t);
        if i != n {
            ans[i] += w;
            seg.set(i, t + s);
        }
    }
    
    for i in 0..n {
        println!("{}", ans[i]);
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
    n: usize,
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
            n: n0,
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

    // false,false,..,true,true,.. の最初のtrue(init: s = 0)
    fn max_right(&self, mut s:usize, mut f:impl FnMut(T) -> bool) -> usize{
        assert!(s <= self.n && f(self.e));
        
        if s == self.n {
            return self.n;
        }

        s += self.size;
        let mut sum = self.e;

        loop {
            s >>= s.trailing_zeros();
            let new = (self.segfunc)(sum, self.data[s]);
            if !f(new) {
                while s < self.size {
                    s *= 2;
                    let res = (self.segfunc)(sum, self.data[s]);
                    if f(res) {
                        sum = res;
                        s += 1;
                    }
                }

                return s - self.size;
            }

            sum = new;
            s += 1;

            if s & s.wrapping_neg() == s {
                break;
            }
        }
        
        self.n
    }

    // ..,true,true,..,false,false の最後のtrue(init: t = n)
    fn min_left(&self, mut t: usize, mut f: impl FnMut(T) -> bool) -> usize {
        assert!(t <= self.n && f(self.e));
        
        if t == 0 {
            return 0;
        }
        t += self.size;
        let mut sum= self.e;

        loop {
            t -= 1;
            t >>= t.trailing_ones();
            t += (t == 0) as usize;
            
            let new = (self.segfunc)(sum, self.data[t]);
            if !f(new) {
                while t < self.size {
                    t = t*2 + 1;
                    let res = (self.segfunc)(sum, self.data[t]);
                    if f(res) {
                        sum = res;
                        t -= 1;
                    }
                }
                return t + 1 - self.size;
            }

            sum = new;
            if t & t.wrapping_neg() == t {
                break;
            }
        }
        
        0
    }
}