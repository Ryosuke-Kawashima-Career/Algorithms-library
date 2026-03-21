use proconio::{input, marker::Chars};
// abc370c
// Q. change S into T in the lexicographically smallest way
// A. categorize by earlier or later 
fn main() {
    input!{s: Chars, t: Chars}
    let n: usize = s.len();
    // lexicographically: s[i] -> t[i]
    let mut smallers: Vec<usize> = Vec::new();
    let mut largers: Vec<usize> = Vec::new();
    for i in 0..n {
        if (s[i] as u8) < (t[i] as u8) {
            smallers.push(i);
        }
        if (s[i] as u8) > (t[i] as u8) {
            largers.push(i);
        }
    }
    let mut x: Vec<Vec<char>> = Vec::new();
    let mut cur: Vec<char> = s.clone();
    // improve
    for &idx_large in largers.iter() {
        cur[idx_large] = t[idx_large];
        x.push(cur.clone());
    }
    // worsen
    for &idx_small in smallers.iter().rev() {
        cur[idx_small] = t[idx_small];
        x.push(cur.clone());
    }
    print_ans(&x);
}

fn print_ans(x: &Vec<Vec<char>>) {
    let m: usize = x.len();
    println!("{}", m);
    if m == 0 {
        return;
    }
    let n: usize = x[0].len();
    for i in 0..m {
        for j in 0..n {
            print!("{}", x[i][j]);
        }
        println!("");
    }
}