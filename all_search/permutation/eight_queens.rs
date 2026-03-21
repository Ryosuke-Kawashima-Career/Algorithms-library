// ALDS_13_A
// brute force attack by permutation
// (row, perm[row]) = (y, x)
fn main() {
    let stdin = std::io::stdin();
    let mut sc = Scanner::new(stdin.lock());
    input!{sc = sc, k: usize, rc: [(i64, i64); k]}
    // (i, perm[i]) denotes a queen
    for perm in (0..8).permutation() {
        if judge(&perm, &rc) {
            show(&perm);
            return;
        }
    }
}

// cols: the columns of the queens, rc: preoccupied queens
fn judge(cols: &Vec<i64>, rc: &Vec<(i64, i64)>) -> bool {
    let mut is_all_placed = true;
    for &(row_occupied, col_occupied) in rc.iter() {
        let mut is_placed = false;
        for row in 0..8 {
            let col: i64 = cols[row];
            if row as i64 == row_occupied && col == col_occupied {
                is_placed = true;
            }
        }
        if !is_placed {
            is_all_placed = false;
        }
    }
    let mut is_not_threatened = true;
    for row1 in 0..8usize {
        for row2 in row1+1..8usize {
            let col1 = cols[row1];
            let col2 = cols[row2];
            if (row1 as i64 - row2 as i64).abs() == (col1 - col2).abs() {
                is_not_threatened = false;
            }
        }
    }
    return is_all_placed && is_not_threatened;
}

fn show(cols: &Vec<i64>) {
    let mut graph: Vec<Vec<char>> = vec![vec!['.'; 8]; 8];
    for row in 0..8 {
        graph[row][cols[row] as usize] = 'Q';
    }

    for i in 0..8 {
        for j in 0..8 {
            print!("{}", graph[i][j]);
        }
        println!("");
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

pub struct PermutationIterator<T: Ord + Clone> {
    li: Vec<T>,
    is_finished: bool,
}

impl <T: Ord + Clone> PermutationIterator<T> {
    pub fn new(mut li: Vec<T>) -> PermutationIterator<T> {
        let is_finished = li.is_empty();
        li.sort();
        PermutationIterator {
            li,
            is_finished,
        }
    }
}

impl <T: Ord + Clone> Iterator for PermutationIterator<T> {
    type Item = Vec<T>;

    // C++ の next_permutation 実装をもとに
    // ref. https://cpprefjp.github.io/reference/algorithm/next_permutation.html
    fn next(&mut self) -> Option<Self::Item> {
        if self.is_finished {
            return None;
        }

        if self.li.len() == 1 {
            self.is_finished = true;
            return Some(self.li.clone());
        }

        let mut i = self.li.len() - 1;
        let res = self.li.clone();

        loop {
            let ii = i;
            i -= 1;
            if self.li[i] < self.li[ii] {
                let mut j = self.li.len() - 1;
                while self.li[i] >= self.li[j] { j -= 1; }

                self.li.swap(i, j);
                self.li[ii..].reverse();
                return Some(res);
            }
            if i == 0 {
                self.li.reverse();
                self.is_finished = true;
                return Some(res);
            }
        }
    }
}

pub trait Permutation<T: Ord + Clone> {
    fn permutation(self) -> PermutationIterator<T>;
}

// Vec<T> に対してのみの実装する
// impl <T: Ord + Clone> Permutation<T> for Vec<T> {
//     fn permutation(self) -> PermutationIterator<T> {
//         PermutationIterator::new(self)
//     }
// }

// IntoIterator を実装するものに対して Permutation を実装する
impl <T: Ord + Clone, I: IntoIterator<Item=T>> Permutation<T> for I {
    fn permutation(self) -> PermutationIterator<T> {
        PermutationIterator::new(self.into_iter().collect())
    }
}