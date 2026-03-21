const MAX: usize = 24 * 60 * 60;
// AOJ 2013 - 大崎
fn main() {
    let stdin = std::io::stdin();
    let mut sc = Scanner::new(stdin.lock());
    loop {
        input!{sc = sc, n: usize}
        if n == 0 {
            break;
        }
        let mut imos: Vec<i64> = vec![0; MAX+1];
        for _ in 0..n {
            input!{sc = sc, start: String, end: String}
            let s: Vec<&str> = start.split(':').collect();
            let t: Vec<&str> = end.split(':').collect();
            let start_h: usize = s[0].parse().unwrap();
            let start_m: usize = s[1].parse().unwrap();
            let start_s: usize = s[2].parse().unwrap();
            let end_h: usize = t[0].parse().unwrap();
            let end_m: usize = t[1].parse().unwrap();
            let end_s: usize = t[2].parse().unwrap();
            let start_time: usize = 60 * 60 * start_h + 60 * start_m + start_s;
            let end_time: usize = 60 * 60 * end_h + 60 * end_m + end_s;
            imos[start_time] += 1;
            imos[end_time] -= 1;
        }
        for i in 1..=MAX {
            imos[i] += imos[i-1];
        }
        let ans = *imos.iter().max().unwrap();
        println!("{}", ans);
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