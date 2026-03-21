use proconio::{input, marker::Chars};
// abc345C
// 1文字ごとでなく，アルファベットごとに分類して計算する
// 文字列Sの(i, j)(i < j)を選んで1回入れ替えてできる文字列の種類数
fn main() {
    input!{s: Chars}
    let n: usize = s.len();
    // a,..,zが何回出現するか
    let mut alphas = vec![0; 26];
    for i in 0..n {
        alphas[s[i] as usize - 'a' as usize] += 1;
    }

    let mut ans: usize = 0;
    // アルファベット26種類から異なる2つ選ぶ
    for i in 0..26 {
        for j in i+1..26 {
            ans += alphas[i] * alphas[j];
        }
    }
    // 1つのアルファベットが2文字以上ある->元の文字列が答えに入る
    for i in 0..26 {
        if alphas[i] >= 2 {
            ans += 1;
            break;
        }
    }
    println!("{}", ans);
}