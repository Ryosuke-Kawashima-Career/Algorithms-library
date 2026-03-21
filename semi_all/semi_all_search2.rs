use proconio::input;
use superslice::Ext;
// 鉄則B14
// 半分全列挙(選んだ数の和がkとなる組み合わせはあるか？)
fn main() {
    input!{n: usize, k: usize, a: [usize; n]}
    let n1: usize = n / 2;
    let n2: usize = n - n1;
    // sliceを使う。
    let mut sums1: Vec<usize> = enumerate_sum(n1, &a[..n1]);
    let mut sums2: Vec<usize> = enumerate_sum(n2, &a[n1..]);
    sums1.sort();
    sums2.sort();

    for &sum1 in sums1.iter() {
        let index: usize = sums2.lower_bound(&(k-sum1));
        if index < sums2.len() && sum1 + sums2[index] == k {
            println!("Yes");
            return;
        }
    }

    println!("No");
}

// sliceを使うので&[usize]
fn enumerate_sum(n: usize, a: &[usize]) -> Vec<usize> {
    let mut sums: Vec<usize> = Vec::new();
    for bit in 0..(1 << n) {
        let mut sum: usize = 0;
        for i in 0..n {
            if bit >> i & 1 == 1 {
                sum += a[i];
            }
        }
        sums.push(sum);
    }

    return sums;
}