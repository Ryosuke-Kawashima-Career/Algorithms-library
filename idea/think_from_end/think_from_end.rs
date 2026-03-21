use proconio::input;
use proconio::marker::Chars;
// 鉄則90A41
// 後ろから考える: 後退解析
fn main() {
    input!{n: usize, s: Chars}
    let mut is_ok: bool = false;
    for i in 0..n {
        if i < n - 2 {
            if s[i] == 'R' && s[i+1] == 'R' && s[i+2] == 'R' {
                is_ok = true;
            }
            if s[i] == 'B' && s[i+1] == 'B' && s[i+2] == 'B' {
                is_ok = true;
            }
        }
    }

    if is_ok {
        println!("Yes");
    } else {
        println!("No");
    }
}