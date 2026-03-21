use proconio::input;
// arc021B: xorでは同じ数を掛けると0になる
// 同じ数をxorするともとに戻る
// Q. b[i] == a[i % n] ^ a[(i+1) % n]
// A. a[i+1] == a[i] ^ b[i]
fn main() {
    input!{l: usize, b: [usize; l]}
    let mut a: Vec<usize> = vec![0; l];
    for i in 1..l {
        a[i] = a[i-1] ^ b[i-1];
    }

    // 境界条件を調べる
    if b[l-1] == a[0] ^ a[l-1] {
        for i in 0..l {
            println!("{}", a[i]);
        }
    } else {
        println!("-1");
    }
}