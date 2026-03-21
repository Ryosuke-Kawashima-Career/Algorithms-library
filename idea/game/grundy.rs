use proconio::input;
const MAX: usize = 100_000;
// 典型A34
// 複数の山からX個またはY個えらぶ。
// grundy数: 0以上の整数の中で前の遷移で使われていない最小の!!!数(Mex)
// grundyのNim和が0なら後手の勝ち
fn main() {
    input!{n: usize, x: usize, y: usize, a: [usize; n]}
    // X個またはY個の二通りしかとり方がないのでgrundy数は0~=2の3通り
    // 石がiこの時のgrundy数
    let mut grundy: Vec<usize> = vec![3; MAX+1];
    // 石が0この時のgrundy数は0: 後手必勝
    grundy[0] = 0;

    for num in 0..=MAX {
        let mut used_grundy: Vec<bool> = vec![false; 3];
        // 前の遷移で使われたgrundy数
        if num >= x {
            used_grundy[grundy[num - x]] = true;
        }
        if num >= y {
            used_grundy[grundy[num - y]] = true;
        }
        for grundy_num in 0..3 {
            if !used_grundy[grundy_num] {
                grundy[num] = grundy_num;
                // 最小の数を選ぶ。
                break;
            }
        }
    }
    // grundy数のnim和を取る
    let mut grundy_nim: usize = 0;
    for i in 0..n {
        grundy_nim ^= grundy[a[i]];
    }
    if grundy_nim == 0 {
        println!("Second");
    } else {
        println!("First");
    }
}