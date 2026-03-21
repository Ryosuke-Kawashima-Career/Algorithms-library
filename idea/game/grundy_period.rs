use proconio::input;
// 鉄則B34
// x=2, y=3の時，grundy数に周期性が生まれる．a[i] <= 10^18
// 手で計算して，規則性を見つける．
// number:grundy = 0:0 1:0 2:1 3:1 4:2, 5:0 6:0 7:1
fn main() {
    input!{n: usize, x: usize, y: usize, a: [usize; n]}
    let grundy = |stone: usize| -> usize {
        match stone % 5 {
            0 => 0,
            1 => 0,
            2 => 1,
            3 => 1,
            _ => 2,
        }
    };

    let mut grundy_nim: usize = 0;
    for i in 0..n {
        grundy_nim ^= grundy(a[i]);
    }

    if grundy_nim == 0 {
        println!("Second");
    } else {
        println!("First");
    }
}