const N1: usize = 1usize.wrapping_neg();
const D8: [(usize, usize); 8] = [
    (N1, N1), (N1, 0), (N1, 1),
    (0, N1), (0, 1),
    (1, N1), (1, 0), (1, 1)
];
// AOJ1160: 島はいくつある?
// グループの数: unionfindでもOK
// 探索されていなかった始点の数 = グループの数
fn main() {
    let stdin = std::io::stdin();
    let mut sc = Scanner::new(stdin.lock());
    loop {
        // 0: sea, 1: land
        input!{sc = sc, w: usize, h: usize, c: [[usize; w]; h]}
        if w == 0 && h == 0 {
            break;
        }
        let mut seen: Vec<Vec<bool>> = vec![vec![false; w]; h];
        let mut ans: usize = 0;
        for i in 0..h {
            for j in 0..w {
                if c[i][j] == 1 && !seen[i][j] {
                    dfs(i, j, &c, &mut seen);
                    ans += 1;
                }
            }
        }
        println!("{}", ans);
    }
}

fn dfs(y0: usize, x0: usize, graph: &Vec<Vec<usize>>, seen: &mut Vec<Vec<bool>>) {
    seen[y0][x0] = true;
    let h: usize = graph.len();
    let w: usize = graph[0].len();
    for &(dy, dx) in D8.iter() {
        let next_y: usize = y0 + dy;
        let next_x: usize = x0 + dx;
        if next_y < h && next_x < w && graph[next_y][next_x] == 1 {
            if seen[next_y][next_x] {
                continue;
            }
            dfs(next_y, next_x, graph, seen);
        }
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