use proconio::{input, marker::Chars};
// abc342c
// 探索の対象をアルファベットに絞る
// ポインターで対応関係を記録する
fn main() {
    input!{n: usize, s: Chars, q: usize, cd: [(char, char); q]}
    let mut pointers: Vec<usize> = (0..26).collect();
    for i in 0..q {
        let before: usize = cd[i].0 as usize - 'a' as usize;
        let after: usize = cd[i].1 as usize - 'a' as usize;
        for j in 0..26 {
            if pointers[j] == before {
                pointers[j] = after;
            }
        }
    }
    for i in 0..n {
        let c: char = (pointers[s[i] as usize - 'a' as usize] as u8 + 'a' as u8) as char;
        print!("{}", c);
    }
}