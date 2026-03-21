use proconio::input;
// 鉄則90A43
// 問題を言い換える。
fn main() {
    input!{n: usize, l: usize}
    let mut ans: usize = 0;
    for _ in 0..n {
        input!{a: usize, b: char}
        let time: usize = if b == 'E' {
            l - a
        } else {
            a
        };
        ans = ans.max(time);
    }

    println!("{}", ans);
}