use proconio::input;
// abc308C
// 式変形によって割り算を回避する
// a: コイン表 b: コイン裏 -> 表の確率: a / (a + b)
fn main() {
    input!{n: usize, ab: [(u128, u128); n]}
    // int, uintをできる限り使う
    let mut success_rates: Vec<(usize, u128, u128)> = Vec::new();
    for i in 0..n {
        let (a, b) = ab[i];
        success_rates.push((i, a, b));
    }
    // 2通りのやり方
    success_rates.sort_by(|x, y| if x.1 * (y.1 + y.2) == y.1 * (x.1 + x.2) {
        x.0.cmp(&y.0)
    } else {
        (y.1 * (x.1 + x.2)).cmp(&(x.1 * (y.1 + y.2)))
    });
    success_rates.sort_by(|(idx1, a1, b1), (idx2, a2, b2)| if a1 * (b1 +b2) == b1 * (a1 + a2) {
        idx1.cmp(&idx2)
    } else {
        (b1 * (a1 + a2)).cmp(&(a1 * (b1 +b2)))
    });
    for i in 0..n {
        print!("{} ", success_rates[i].0 + 1);
    }
    println!("");
}