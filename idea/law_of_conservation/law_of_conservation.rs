use proconio::input;
use proconio::marker::Chars;
// 鉄則90A45
// 不変量に注目する。
// W: 0, B: 1, R: 2とし、合計をscoreとする。
// score(mod3)が不変量となる。
fn main() {
    input!{n: usize, c: char, a: Chars}
    let char_to_num = |x: char| -> usize {
        if x == 'W' {
            0
        } else if x == 'B' {
            1
        } else {
            2
        }
    };
    let c_mod3: usize = char_to_num(c);
    let score: usize = a.iter().map(|&x| char_to_num(x)).sum::<usize>();
    let score_mod3 = score % 3;

    if c_mod3 == score_mod3 {
        println!("Yes");
    } else {
        println!("No");
    }
}