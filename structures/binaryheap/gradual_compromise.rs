use proconio::input;
use std::collections::BinaryHeap;
// ABC440E: Best First Search
// Q. 重複組み合わせで得られるスコアの上位X個を出力する
// A. PriorityQueueでスコアを管理する => Best First Search
// A. 最も美味しさの和が大きな選び方から少しずつ妥協していく過程を、プライオリティーキューを用いてシミュレーションする
// A. 美味しさの和と選び方の組(cookie_0, cookie_1, cookie_N-1)をプライオリティーキューで管理しながら、キューから要素を取り出した際に一段階妥協した選び方(cookie_0, cookie_1 - 1, cookie_N-1)をキューに追加することで、美味しさの和の順に取り出すことができます。
fn main() {
    // Choose K cookies from N cookies
    input! {n: usize, k: usize, x: usize, mut a: [i64; n]}
    a.sort_by(|a, b| b.cmp(a));
    let mut scores: BinaryHeap<(i64, usize, usize, usize)> = BinaryHeap::new();
    // (score, position, cookies[position-1], cookies[position])
    scores.push((a[0] * k as i64, 0, 0, k));
    // (現在のスコア, 現在のクッキー, 自分よりスコアが大きいクッキーの数, 現在のクッキーの数)
    let mut answer: Vec<i64> = Vec::new();
    // Gradually Compromise with the best choice
    while let Some((cur_score, cur_pos, num_of_prev, num_of_cur)) = scores.pop() {
        answer.push(cur_score);
        if answer.len() == x {
            break;
        }
        // Do not move the current position
        if num_of_prev > 0 && cur_pos > 0 && cur_pos < n {
            scores.push((
                cur_score - a[cur_pos - 1] + a[cur_pos],
                cur_pos,
                num_of_prev - 1,
                num_of_cur + 1,
            ));
        }
        // Move the current position forward
        if cur_pos != n - 1 {
            scores.push((
                cur_score - a[cur_pos] + a[cur_pos + 1],
                cur_pos + 1,
                num_of_cur - 1,
                1,
            ));
        }
    }
    for ans in answer {
        println!("{}", ans);
    }
}
