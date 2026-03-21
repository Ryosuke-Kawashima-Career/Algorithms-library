use proconio::{input, marker::Chars};
// abc349C
// 貪欲法を使うのでは？(できるだけ左の文字を使う)
// 1. narita -> NRT
// 2. losangeles -> LAX
fn main() {
    input!{s: Chars, t: Chars}
    let n: usize = s.len();

    let is_code = if t[2] == 'X' {
        is_matched(&s, &t[..2])
    } else {
        is_matched(&s, &t)
    };
    if is_code {
        println!("Yes");
    } else {
        println!("No");
    }
}

// 文字列s,tが一致するか?
fn is_matched(s: &[char], t: &[char]) -> bool {
    let n: usize = s.len();
    let mut cur_index: usize = 0;
    for &letter_t in t.iter() {
        // sとtが一致するまでループ
        while cur_index < n 
        && s[cur_index] as usize - 'a' as usize != letter_t as usize - 'A' as usize 
        {
            cur_index += 1;
        }
        if cur_index == n {
            return false;
        }
        // 文字を1つ使ったのでindexを1つ進める
        cur_index += 1;
    }
    return true;
}