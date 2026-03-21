use proconio::input;
use proconio::marker::Chars;
// 典型84
// 余事象と座標圧縮(ランレングス圧縮)
// oとxを両方含む文字列 <-> oかx片方を含む文字列
fn main() {
    input!{n: usize, s: Chars}
    // 余事象
    let mut complementary: usize = 0;
    let compressed: Vec<(char, usize)> = rle(&s);

    for &(_, value) in compressed.iter() {
        if value > 1 {
            complementary += value * (value - 1) / 2;
        }
    }

    let all: usize = n * (n-1) / 2;
    let ans: usize = all - complementary;
    println!("{}", ans);
}
// ランレングス圧縮(iとjの尺取り虫)
fn rle(s: &Vec<char>) -> Vec<(char, usize)> {
    let mut res: Vec<(char, usize)> = Vec::new();
    let n: usize = s.len();

    let mut i: usize = 0;
    while i < n {
        let mut j: usize = i;
        while j < n && s[i] == s[j] {
            j += 1;
        }

        res.push((s[i], j-i));
        i = j;
    }

    return res;
}