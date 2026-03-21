use proconio::input;
use proconio::marker::Chars;
// diverta2019c
// ペアリング
// 文字列を分類する: 1. b-start-a-end 2. a-end 3. b-start
fn main() {
    input!{n: usize, s: [Chars; n]}
    let mut ab_nums: usize = 0;
    // Aの数: type1 + type2
    // Bの数: type1 + type3
    // 上界は、min(Aの数, Bの数)
    let mut b_start_num: usize = 0;
    let mut a_end_num: usize = 0;
    let mut b_start_a_end_num: usize = 0;

    for i in 0..n {
        for j in 1..s[i].len() {
            if s[i][j-1] == 'A' && s[i][j] == 'B' {
                ab_nums += 1;
            }
        }

        let last: usize = s[i].len() - 1;
        if s[i][0] == 'B' && s[i][last] == 'A' {
            b_start_a_end_num += 1;
        } else if s[i][0] == 'B' {
            b_start_num += 1;
        } else if s[i][last] == 'A' {
            a_end_num += 1;
        }
    }

    // 上界が満たされないコーナーケース
    if b_start_num == 0 && a_end_num == 0 {
        if b_start_a_end_num > 0 {
            ab_nums += b_start_a_end_num - 1;
        }
    } else {
        ab_nums += b_start_a_end_num + b_start_num.min(a_end_num);
    }

    println!("{}", ab_nums);
}