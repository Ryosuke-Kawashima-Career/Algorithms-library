use proconio::{input, marker::Chars};
// abc386c
// Q. 文字列Sに対して以下の操作を行って、文字列Tと一致させられるか判定
// A. 文字列の長さで場合分け
fn main() {
    input!{_k: usize, s: Chars, t: Chars}
    if is_adjustable(&s, &t) {
        println!("Yes");
    } else {
        println!("No");
    }
}

fn is_adjustable(s: &Vec<char>, t: &Vec<char>) -> bool {
    let s_len: usize = s.len();
    let t_len: usize = t.len();
    if s_len + 1 == t_len {
        // insert a chracter in S
        let mut difs: usize = 0;
        let mut cur_index: usize = 0;
        for i in 0..t_len {
            if cur_index < s_len && s[cur_index] != t[i] {
                difs += 1;
                continue;
            }
            cur_index += 1;
        }
        if difs < 2 {
            return true;
        } else {
            return false;
        }
    } else if s_len == t_len + 1 {
        // delete a character in S
        let mut difs: usize = 0;
        let mut cur_index: usize = 0;
        for i in 0..s_len {
            if cur_index < t_len && s[i] != t[cur_index] {
                difs += 1;
                continue;
            }
            cur_index += 1;
        }
        if difs < 2 {
            return true;
        } else {
            return false;
        }
    } else if s_len == t_len {
        // change a character in S
        let mut difs: usize = 0;
        for i in 0..s_len {
            if s[i] != t[i] {
                difs += 1;
            }
        }
        if difs < 2 {
            return true;
        } else {
            return false;
        }
    }
    return false;
}