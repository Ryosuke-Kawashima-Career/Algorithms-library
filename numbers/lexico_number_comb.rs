use proconio::input;
// abc335b
// 非負整数の組の辞書順
// 非負整数の組(x,y,z)が(x′,y′,z′)より辞書順で小さい
// 1. x<x′
// 2. x=x′ かつ y<y′
// 3. x=x′ かつ y=y′ かつ z<z′
fn main() {
    input!{n: usize}
    // (x, y, z)を辞書順に並べる
    for x in 0..=n {
        for y in 0..=n {
            for z in 0..=n {
                if x + y + z <= n {
                    println!("{} {} {}", x, y, z);
                }
            }
        }
    }
}