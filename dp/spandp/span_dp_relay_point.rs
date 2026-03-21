// AOJ 1611 ダルマ落とし
// 区間DP: lとrの中点kを計算する(k: l+1..r)
fn main() {
    let stdin = std::io::stdin();
    let mut sc = Scanner::new(stdin.lock());
    loop {
        input!{sc = sc, n: usize}
        if n == 0 {
            break;
        }
        input!{sc = sc, w: [i64; n]}
        let mut dp: Vec<Vec<usize>> = vec![vec![0; n+1]; n+1];
        // 初期値
        for i in 0..n-1 {
            if (w[i]-w[i+1]).abs() < 2 {
                dp[i][i+2] = 2;
            }
        }

        for span in 2..=n {
            for l in 0..n {
                let r: usize = l + span;
                if r > n {
                    break;
                }
                // k: lとrの中点
                for k in l+1..r {
                    dp[l][r] = dp[l][r].max(dp[l][k] + dp[k][r]);
                }
                // だるまの中身がすべて取り除かれるとき
                if dp[l+1][r-1] == r - l - 2 && (w[l]-w[r-1]).abs() < 2 {
                    dp[l][r] = dp[l][r].max(dp[l+1][r-1]+2);
                }
            }
        }
        
        println!("{}", dp[0][n]);
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