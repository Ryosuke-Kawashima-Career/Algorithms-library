use proconio::input;
// abc319d
// 文章をm行に表示するための最小の横幅
// 横幅を仮定すると文章がm行に収まるか判定できる
// 答えの二分探索
fn main() {
    input!{n: usize, m: usize, original_words: [usize; n]}
    // あらかじめ単語の先頭に空白を付け足す
    let words: Vec<usize> = original_words.iter().map(|&word| 1 + word).collect::<_>();
    // l: 収まらない, r: 収まる
    let mut l: usize = *words.iter().max().unwrap() - 1;
    let mut r: usize = words.iter().sum();
    while r - l > 1 {
        let mid: usize = (l + r) / 2;
        if calc_row(mid, &words) <= m {
            r = mid;
        } else {
            l = mid;
        }
    }
    // 先頭の空白を消す
    println!("{}", r - 1);
}

fn calc_row(width: usize, words: &Vec<usize>) -> usize {
    let mut rows: usize = 1;
    let mut cur_x: usize = 0;
    for &word in words.iter() {
        cur_x += word;
        if cur_x > width {
            cur_x = word;
            rows += 1;
        }
    }
    return rows;
}