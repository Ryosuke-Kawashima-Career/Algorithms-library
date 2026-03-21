// AOJ 1149 - ケーキカット
// 長方形を幅と高さで管理する
fn main() {
    let stdin = std::io::stdin();
    let mut sc = Scanner::new(stdin.lock());
    loop {
        input!{sc = sc, n: usize, w: usize, d: usize}
        if n == 0 && w == 0 && d == 0 {
            break;
        }
        input!{sc = sc, ps: [(Usize1, usize); n]}
        // stackで番号を管理する
        let mut rectangles = Vec::new();
        rectangles.push((w, d));
        for i in 0..n {
            let (width, height) = rectangles.remove(ps[i].0);
            let new_rectangles = calc_rect(width, height, ps[i].1);
            rectangles.push(new_rectangles[0]);
            rectangles.push(new_rectangles[1]);
        }
        let mut areas: Vec<usize> = Vec::new();
        for &(width, height) in rectangles.iter() {
            areas.push(width * height);
        }
        areas.sort();
        for i in 0..areas.len() {
            if i == 0 {
                print!("{}", areas[i]);
            } else {
                print!(" {}", areas[i]);
            }
        }
        println!("");
    }
}

fn calc_rect(width: usize, height: usize, length: usize) -> Vec<(usize, usize)> {
    let mut res: Vec<(usize, usize)> = Vec::new();
    // 繰り返し構造をmodで計算する
    let mut modu: usize = width + height;
    let cut_len: usize = length % modu;
    // 場合分け
    if cut_len <= width {
        let width1: usize = cut_len;
        let width2: usize = width - cut_len;
        // 小さい順に並べる
        res.push((width1.min(width2), height));
        res.push((width1.max(width2), height));
    } else {
        let height1: usize = cut_len - width;
        let height2: usize = height - height1;
        res.push((width, height1.min(height2)));
        res.push((width, height1.max(height2)));
    }
    return res;
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