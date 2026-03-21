use proconio::input;
// 鉄則90A36
// parity(偶奇)
fn main() {
    input!{n: usize, k: usize}
    let distance: usize = 2*(n-1);
    let parity: usize = distance % 2;
    if distance <= k && parity == k % 2 {
        println!("Yes");
    } else {
        println!("No");
    }
}