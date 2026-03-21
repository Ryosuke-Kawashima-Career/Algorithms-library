use proconio::{input, marker::Chars};
// abc307D
//「いま注目している位置より前にある最も近い(を探す」ことが,O(1)時間でできる
// 処理全体のどこに時間がかかっているかを見極め、それを改善するためにはどうすればよいかを,1ステップずつ考える
fn main() {
    input!{n: usize, s: Chars}
    // ()をstackで管理する
    let mut stack: Vec<char> = Vec::new();
    // 使われていない(の数
    let mut depth: usize = 0;
    
    for i in 0..n {
        if s[i] == '(' {
            stack.push(s[i]);
            depth += 1;
        } else if s[i] == ')' {
            if depth == 0 {
                stack.push(s[i]);
            } else {
                while let Some(tail) = stack.last() {
                    if *tail != '(' {
                        stack.pop();
                    } else {
                        break;
                    }
                }
                // (をpop
                stack.pop();
                depth -= 1;
            }
        } else {
            stack.push(s[i]);
        }
    }

    let ans = stack.iter().map(|x| x.to_string()).collect::<Vec<String>>().join("");
    println!("{}", ans);
}