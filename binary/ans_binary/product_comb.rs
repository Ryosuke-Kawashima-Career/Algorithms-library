use proconio::input;
// arc037c: 億マス計算
// Q. a * b　の数の中でk番目に小さい数は?
// 答えの二分探索
fn main() {
    input!{n: usize, k: usize, a: [usize; n], mut b: [usize; n]}
    b.sort();
    let mut l: usize = 0;
    let mut r: usize = 1 << 60;
    while r - l > 1 {
        let mid: usize = (l + r) / 2;
        if count_smaller(mid, &a, &b) < k {
            l = mid;
        } else {
            r = mid;
        }
    }

    println!("{}", l);
}

// numより小さい組み合わせを計算する
fn count_smaller(num: usize, a: &Vec<usize>, b: &Vec<usize>) -> usize {
    let mut res: usize = 0;
    let n: usize = a.len();
    for i in 0..n {
        // 割り算の使用を避ける
        let num_smaller: usize = b.partition_point(|&x| a[i] * x < num);
        res += num_smaller;
    }
    return res;
}