// AOJ1193: 連鎖消滅パズル
fn main() {
    let stdin = std::io::stdin();
    let mut sc = Scanner::new(stdin.lock());
    loop {
        input!{sc = sc, h: usize}
        if h == 0 {
            break;
        }
        input!{sc=sc, mut graph: [[usize; 5]; h]}
        let mut score: usize = 0;
        while can_erase(3, &mut graph, &mut score) {
            update(&mut graph);
        }
        println!("{}", score);
    }
}

fn can_erase(k: usize, graph: &mut Vec<Vec<usize>>, score: &mut usize) -> bool {
    let h: usize = graph.len();
    let w: usize = graph[0].len();
    let mut erased: bool = false;
    // height is fixed
    for i in (0..h).rev() {
        // RLE: Runtime length encoding
        // (block, num)
        let mut rle: Vec<(usize, usize)> = Vec::new();
        let mut left: usize = 0;
        while left < w {
            let mut right = left;
            while right < w && graph[i][left] == graph[i][right] {
                right += 1;
            }
            rle.push((graph[i][left], right-left));
            left = right;
        }
        
        let mut start: usize = 0;
        for &(number, span) in rle.iter() {
            // prevent index out of bounds
            let end: usize = (start+span).min(w);
            if number > 0 && span >= k {
                *score += number * span;
                erased = true;
                for update_j in start..end {
                    graph[i][update_j] = 0;
                }
            }
            start = end;
        }
        
    }
    return erased;
}

// 0 means empty
fn update(graph: &mut Vec<Vec<usize>>) {
    let h: usize = graph.len();
    let w: usize = graph[0].len();
    for j in 0..w {
        let mut remain: Vec<usize> = Vec::new();
        for i in (0..h).rev() {
            if graph[i][j] != 0 {
                remain.push(graph[i][j]);
            }
        }
        for remain_i in 0..h {
            if remain_i < remain.len() {
                graph[h-remain_i-1][j] = remain[remain_i];
            } else {
                graph[h-remain_i-1][j] = 0;
            }
        }
    }
}

fn debug(graph: &Vec<Vec<usize>>) {
    let h: usize = graph.len();
    let w: usize = graph[0].len();
    for i in 0..h {
        for j in 0..w {
            print!("{} ", graph[i][j]);
        }
        println!("");
    }
    println!("");
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