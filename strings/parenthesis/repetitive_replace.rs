use proconio::{input, marker::Chars};
// abc394c
// Q. As long as the string contains "WA" as a contiguous substring, repeat replacing the leftmost one with "AC".
// A. stack
fn main() {
    input!{s: Chars}
    let n: usize = s.len();
    let mut stack: Vec<char> = Vec::new();
    for i in 0..n {
        stack.push(s[i]);
        let mut cur_pos: usize = stack.len();
        while cur_pos >= 2 && stack[cur_pos-2] == 'W' && stack[cur_pos-1] == 'A' {
            stack[cur_pos-2] = 'A';
            stack[cur_pos-1] = 'C';
            cur_pos -= 1;
        }
    }
    for i in 0..n {
        print!("{}", stack[i]);
    }
    println!("");
}