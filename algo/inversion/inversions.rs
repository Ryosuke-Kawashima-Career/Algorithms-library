use proconio::input;
// 鉄則B59
// 転倒数: 1≤i<j≤N かつAi>Ajを満たす整数の組(i,j)の個数
// 前からfor文を回して整数xが何回出現したか記録する配列を用意する。
// Aは(0,1,2,…,N-1) を並べ替えた順列である(座標圧縮すればどの配列においても転倒数を計算できる。)
fn main() {
    input!{n: usize, a: [i64; n]}
    // 和を求める関数
    fn inv_func(a: i64, b: i64) -> i64 {
        a + b
    }
    let mut segtree = SegTree::new(&vec![0; n], inv_func, 0);
    let mut inversions: i64 = 0;

    // 通常なら配列Aをあらかじめ座標圧縮する。
    for i in 0..n {
        let ai: usize = (a[i] - 1) as usize;
        // Ai以上の数
        inversions += segtree.prod(ai, n-1);
        // Aiの出現回数を加算する。
        segtree.set(ai.saturating_sub(1), segtree.get(ai.saturating_sub(1))+1);
    }

    println!("{}", inversions);
}

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