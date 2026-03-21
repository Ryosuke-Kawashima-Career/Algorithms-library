use proconio::input;
// abc438_c
// Q. If there is a 4 consecutive numbers in the array, these elements are removed.
// A. Use stack.
fn main() {
    input! {n: usize, a: [usize; n]}
    let mut stack: Vec<usize> = Vec::new();
    for i in 0..n {
        stack.push(a[i]);
        if stack.len() >= 4 {
            if stack[stack.len() - 1] == stack[stack.len() - 2]
                && stack[stack.len() - 2] == stack[stack.len() - 3]
                && stack[stack.len() - 3] == stack[stack.len() - 4]
            {
                for _ in 0..4 {
                    stack.pop();
                }
            }
        }
    }
    println!("{}", stack.len());
}
