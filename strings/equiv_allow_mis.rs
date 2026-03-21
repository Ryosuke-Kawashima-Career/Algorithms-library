use proconio::input;
use proconio::marker::Chars;
// abc324c
// check by allowing mistakes
fn main() {
    input!{n: usize, target: Chars, s: [Chars; n]}
    let mut ans: Vec<usize> = Vec::new();
    for i in 0..n {
        if is_equiv(&s[i], &target) {
            ans.push(i+1);
        }
    }
    let k: usize = ans.len();
    println!("{}", k);
    for i in 0..k {
        print!("{} ", ans[i]);
    }
}

// len(s) <= len(t)
fn is_equiv(s: &Vec<char>, t: &Vec<char>) -> bool {
    if s.len() > t.len() {
        return is_equiv(t, s);
    }
    if s.len() + 1 < t.len() {
        return false;
    }
    let mut miss: usize = 0;
    let mut i: usize = 0;
    let mut j: usize = 0;
    while i < s.len() {
        if s[i] == t[j] {
            i += 1;
            j += 1;
        } else {
            miss += 1;
            if miss > 1 {
                return false;
            }
            if s.len() == t.len() {
                i += 1;
            }
            // 長いほうのindexを進める
            j += 1;
        }
    }
    return true;
}