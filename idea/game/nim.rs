use proconio::input;
// 典型A33
// 複数の山から好きな数だけとる。
// ものまね戦法で後手が勝つ。
// nim和(sum ^= val) == 0で一般化する。
fn main() {
    input!{n: usize, mounts: [usize; n]}
    let mut nim: usize = 0;

    for i in 0..n {
        nim ^= mounts[i];
    }

    if nim == 0 {
        println!("Second");
    } else {
        println!("First");
    }
}