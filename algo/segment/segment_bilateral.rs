use proconio::input;
// abc349D
// セグメント木の区間の取り方を応用する
// Span(l, r) = [l, r) = Span(2^i*j, 2^i*(j+1))
// 多くのセグメント木の実装では区間クエリを下側から順番に見ていきながら処理するが，それと同じことをすればよい
// 下から順番に見ていって, 左側と右側それぞれで長さ2^iの数列を選択するかを決めていく
// 左側で長さ2iの数列を選択する条件: 1. Lのi-bit目が立っている 2. L<R
fn main() {
    input!{l: usize, r: usize}
    let answer: Vec<(usize, usize)> = segments(l, r);
    print_answer(&answer);
}

fn segments(mut l: usize, mut r: usize) -> Vec<(usize, usize)> {
    let mut left: Vec<(usize, usize)> = Vec::new();
    let mut right: Vec<(usize, usize)> = Vec::new();
    let mut i: usize = 0;

    while l < r {
        if (l >> i) & 1 == 1 {
            left.push((l, l + (1 << i)));
            l += 1 << i;
        }
        if (r >> i) & 1 == 1 {
            right.push((r - (1 << i), r));
            r -= 1 << i;
        }
        i += 1;
    }

    right.reverse();
    left.append(&mut right);

    return left;
}

fn print_answer(answer: &Vec<(usize, usize)>) {
    let k: usize = answer.len();
    println!("{}", k);
    for &(l, r) in answer.iter() {
        println!("{} {}", l, r);
    }
}