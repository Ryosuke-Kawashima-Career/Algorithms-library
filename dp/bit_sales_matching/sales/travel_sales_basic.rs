const INF: i64 = 1 << 60;
// Aizu DPL_2_A
fn main() {
    input!{v: usize, e: usize, edges: [(usize, usize, i64); e]}
    let mut graph: Vec<Vec<(usize, i64)>> = vec![vec![]; v];
    for &(from, to, cost) in edges.iter() {
        graph[from].push((to, cost));
    }
    let bits: usize = 1 << v;
    // min dist = dp[cur position][visited]
    let mut dp: Vec<Vec<i64>> = vec![vec![INF; bits]; v];
    // 一周するので0を始点としてよい
    dp[0][0] = 0;
    // bit -> node
    for bit in 0..bits {
        for i in 0..v {
            for &(next, cost) in graph[i].iter() {
                // if next is not visited
                if bit >> next & 1 == 0 {
                    let next_state: usize = bit | (1 << next);
                    dp[next][next_state] = dp[next][next_state].min(dp[i][bit] + cost);
                }
            }
        }
    }

    if dp[0][bits-1] == INF {
        println!("-1");
    } else {
        println!("{}", dp[0][bits-1]);
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
