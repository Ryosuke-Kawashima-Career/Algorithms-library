use proconio::input;
// abc134d
// 後ろから考える
// 選択肢が少ない方から見て貪欲
// Ex. 「時間が経つにつれて取れる選択肢が少なくなる」「前半に選択したものが後半に影響して選択肢が少なくなる」
// Q. i(1..=n)の倍数が書かれた箱に入っているボールの個数の和を2で割った余りがaiになるようにi番目にボールを一つ入れるか決める
// (a + b) % 2 == a ^ b == abs(a-b) % 2 
fn main() {
    input!{n: usize, mut a: [usize; n]}
    let mut answer: Vec<usize> = Vec::new();
    // １．後ろから考えるとボールの入れ方が一意に決まる
    for i in (1..=n).rev() {
        if a[i-1] == 1 {
            answer.push(i);
            // ２．倍数の逆なので約数
            for &div in get_divs(i).iter() {
                a[div-1] ^= 1;
            }
        }
    }

    println!("{}", answer.len());
    for &ans in answer.iter().rev() {
        print!("{} ", ans);
    }
}

fn get_divs(num: usize) -> Vec<usize> {
    let mut res: Vec<usize> = Vec::new();
    let mut div: usize = 1;
    while div * div <= num {
        if num % div == 0 {
            res.push(div);
            if div != num / div {
                res.push(num / div);
            }
        }
        div += 1;
    }
    return res;
}