#[allow(non_snake_case)]
use proconio::input;
const DIGIT: i128 = 60;
// abc347D
// xorにおけるpopcountの条件式
// (a + b + c) % 2 == 0 && a + b + c <= 2 * DIGIT && a <= b + c && b <= c + a && c <= a + b
// 二進法の桁における方程式
// 1. popcount(X)=a ⟺ n1,0+n1,1=a
// 2. popcount(Y)=b ⟺ n0,1+n1,1=b
// 3. X^Y=C ⟹ n1,0+n0,1=popcount(C)
fn main() {
    // a = popcount(x), b = popcount(y), c = x ^ y
    input!{a: i128, b: i128, C: i128}
    // X,Yを2進法で表したときのk桁目(0≤k<DIGIT)がそれぞれi,j(i,j∈{0,1})となるようなkの個数をni,j
    let c = popcount(C);
    // ni,j >= 0
    if (a + b + c) % 2 != 0 || a + b + c > 2 * DIGIT || a > b + c || b > c + a || c > a + b {
        println!("-1");
        return;
    }
    // n0,0 + n0,1 + n1,0 + n1,1 = DIGIT
    let mut n00 = DIGIT - (a + b + c) / 2;
    let mut n01 = (-a + b + c) / 2;
    let mut n10 = (a - b + c) / 2;
    let mut n11 = (a + b - c) / 2;

    let mut x: i128 = 0;
    let mut y: i128 = 0;
    // 上の bit から見る
    for digit in (0..DIGIT).rev() {
        // 1 bit ずつ上に持ち上げて
        x <<= 1;
        y <<= 1;
        // c: intの時,()を付けてbit演算をする
        if (C >> digit) & 1 == 1 {
            if n10 > 0 {
                // bitを立てる
                x += 1;
                n10 -= 1;
            } else if n01 > 0 {
                y += 1;
                n01 -= 1;
            }
        } else {
            if n00 > 0 {
                n00 -= 1;
            } else if n11 > 0 {
                x += 1;
                y += 1;
                n11 -= 1;
            }
        }
    }
    println!("{} {}", x, y);
}

fn popcount(mut n: i128) -> i128 {
    let mut ones = 0;
    while n > 0 {
        ones += n & 1;
        n >>= 1;
    }
    return ones;
}