use proconio::{input, marker::Chars};
// Q. judge whether the given string can be empty by procedures of tetris
// E.g. ([])<>() -> ()<>() -> ""
// A. use stack and pop two corresponding elements
fn main() {
    input!{s: Chars}
    let n: usize = s.len();
    if n % 2 == 1 {
        println!("No");
        return;
    }
    let mut stack: Vec<char> = Vec::new();
    for i in 0..n {
        stack.push(s[i]);
        let mut length: usize = stack.len();
        while length >= 2 
        && ((stack[length-2] == '(' && stack[length-1] == ')')
        || (stack[length-2] == '[' && stack[length-1] == ']')
        || (stack[length-2] == '<' && stack[length-1] == '>')) 
        {
            stack.pop();
            stack.pop();
            length -= 2;
        }
    }
    if stack.len() == 0 {
        println!("Yes");
    } else {
        println!("No");
    }
}