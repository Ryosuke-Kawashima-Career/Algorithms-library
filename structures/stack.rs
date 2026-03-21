use proconio::input;
use proconio::marker::Chars;
// abc328d
// テトリスなのでスタックを使う。
fn main() {
    input!{s: Chars}
    let mut stack: Vec<char> = Vec::new();

    for &c in s.iter() {
        stack.push(c);
        
        while stack.len() >= 3 
        && &stack[(stack.len()-3)..stack.len()] == &['A', 'B', 'C'] {
            for _ in 0..3 {
                stack.pop();
            }
        }
    }

    let ans: String = stack.iter().collect::<String>();
    println!("{}", ans);
}

fn example() {
    if let Some(num) = stack.last() {
    } else {}
    if let Some(num) = stack.last_mut() {
        *num += 1;
    } else {}
    // erase a value
    match graph[next].binary_search(&v) {
        Ok(removal_index) => graph[next].remove(removal_index),
        Err(_) => {continue} // value not contained.
    };
}