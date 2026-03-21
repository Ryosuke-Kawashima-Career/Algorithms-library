// AOJ1127: Building a Space Station
//　円の位置関係の公式 d=距離, r1, r2=半径
// dが次第に増加する
// d < |r1 - r2|: 一つの円が他方に内包される
// d = |r1 - r2|: 内接
// |r1 - r2| < d < r1 + r2: 二つの円が交わる
// d = r1 + r2: 外接
// r1 + r2 < d: 円が離れすぎて交わらない
fn main() {
    let stdin = std::io::stdin();
    let mut sc = Scanner::new(stdin.lock());
    // クラスカル法で最小全域木を解くだけ
    loop {
        input!{sc=sc, n: usize}
        if n == 0 {
            break;
        }
        input!{sc=sc, xyzr: [(f64, f64, f64, f64); n]}
        let dist = |i: usize, j: usize| -> f64 {
            let x_square: f64 = (xyzr[i].0 - xyzr[j].0) * (xyzr[i].0 - xyzr[j].0);
            let y_square: f64 = (xyzr[i].1 - xyzr[j].1) * (xyzr[i].1 - xyzr[j].1);
            let z_square: f64 = (xyzr[i].2 - xyzr[j].2) * (xyzr[i].2 - xyzr[j].2);
            (x_square + y_square + z_square).sqrt()
        };
        let mut uf = UnionFind::new(n);
        let mut edges: Vec<(usize, usize, f64)> = Vec::new();
        for i in 0..n {
            for j in i+1..n {
                let dist_ij: f64 = dist(i, j);
                // 球体が交わらないので
                if dist_ij > xyzr[i].3 + xyzr[j].3 {
                    edges.push((i, j, dist_ij - (xyzr[i].3 + xyzr[j].3)));
                } else {
                    edges.push((i, j, 0.0));
                }
            }
        }
        edges.sort_by(|x, y| x.2.partial_cmp(&y.2).unwrap());
        let mut ans = 0.0;
        for &(node1, node2, cost) in edges.iter() {
            if !uf.equiv(node1, node2) {
                ans += cost;
                uf.union(node1, node2);
            }
        }
        // 小数点以下３桁のみを表示する
        println!("{:.3}", ans);
    }
}

pub struct UnionFind {
    parent: Vec<usize>,
    size: Vec<usize>,
    rank: Vec<usize>,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        UnionFind {
            parent: (0..n).collect(),
            size: vec![1; n],
            rank: vec![0; n],
        }
    }

    // 根を返却
    pub fn root(&mut self, x: usize) -> usize {
        // parentが自分自身の場合は根
        if self.parent[x] == x {
            return x;
        }
        // 経路圧縮
        self.parent[x] = self.root(self.parent[x]);
        self.parent[x]
    }

    // xとyが同じ根か判定
    pub fn equiv(&mut self, x: usize, y: usize) -> bool {
        self.root(x) == self.root(y)
    }

    // xとyを合体
    pub fn union(&mut self, x: usize, y: usize) -> bool {
        let mut rx = self.root(x);
        let mut ry = self.root(y);
        // 既に同じ
        if rx == ry {
            return false;
        }

        // ryのrankが小さくなるように調整
        // ここを省略するとrxが親になる
        if self.rank[rx] < self.rank[ry] {
            std::mem::swap(&mut rx, &mut ry);
        }
        // ryの根をrxにする
        self.parent[ry] = rx;
        // rxのrank調整
        if self.rank[rx] == self.rank[ry] {
            self.rank[rx] += 1;
        }
        self.size[rx] += self.size[ry];
        true
    }

    // xのグループの要素数
    pub fn size(&mut self, x: usize) -> usize {
        let root = self.root(x);
        self.size[root]
    }

    // 連結かどうかを返却する
    pub fn is_linked(&mut self) -> bool {
        self.size(0) == self.size.len()
    }

    // グループの数を計算
    pub fn groups(&mut self) -> usize {
        let mut res: usize = 0;
        for x in 0..self.size.len() {
            // グループの数 = 根の数
            if x == self.root(x) {
                res += 1;
            }
        }
        res
    }
}

#[macro_export]
macro_rules! input{
    (sc=$sc:expr,$($r:tt)*)=>{
        input_inner!{$sc,$($r)*}
    };
    ($($r:tt)*)=>{
        let mut sc=Scanner::new(std::io::stdin().lock());
        input_inner!{sc,$($r)*}
    };
}

#[macro_export]
macro_rules! input_inner{
    ($sc:expr)=>{};
    ($sc:expr,)=>{};
    ($sc:expr,$var:ident:$t:tt$($r:tt)*)=>{
        let $var=read_value!($sc,$t);
        input_inner!{$sc $($r)*}
    };
    ($sc:expr,mut $var:ident:$t:tt$($r:tt)*)=>{
        let mut $var=read_value!($sc,$t);
        input_inner!{$sc $($r)*}
    };
}

#[macro_export]
macro_rules! read_value{
    ($sc:expr,($($t:tt),*))=>{
        ($(read_value!($sc,$t)),*)
    };
    ($sc:expr,[$t:tt;$len:expr])=>{
        (0..$len).map(|_|read_value!($sc,$t)).collect::<Vec<_>>()
    };
    ($sc:expr,Chars)=>{read_value!($sc,String).chars().collect::<Vec<char>>()};
    ($sc:expr,Usize1)=>{read_value!($sc,usize)-1};
    ($sc:expr,$t:ty)=>{$sc.next::<$t>()};
}
pub struct Scanner {
    s: Box<str>,
    input: std::iter::Peekable<std::str::SplitAsciiWhitespace<'static>>,
}
impl Scanner {
    pub fn new<R: std::io::Read>(mut reader: R) -> Self {
        let s = {
            let mut s = String::new();
            reader.read_to_string(&mut s).unwrap();
            s.into_boxed_str()
        };
        let mut sc = Scanner {
            s,
            input: "".split_ascii_whitespace().peekable(),
        };
        use std::mem;
        let s: &'static str = unsafe { mem::transmute(&*sc.s) };
        sc.input = s.split_ascii_whitespace().peekable();
        sc
    }
    #[inline]
    pub fn next<T: std::str::FromStr>(&mut self) -> T
    where
        T::Err: std::fmt::Debug,
    {
        self.input
            .next()
            .unwrap()
            .parse::<T>()
            .expect("Parse error")
    }
}