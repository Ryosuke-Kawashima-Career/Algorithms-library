use proconio::input;
// abc388e
// Q. find the maximum non-negative integer K for which the following is possible:
// From the N mochi, choose 2K of them to form K pairs. For each pair, place one mochi on top of the other, to make K kagamimochi.
// Given two mochi A and B, with sizes a and b respectively, you can make one kagamimochi (a stacked rice cake) by placing mochi A on top of mochi B if and only if a is at most half of b.
// A. 貪欲法: 先頭K個と末尾K個の餅を使う(the most extreme case)
fn main() {
    input!{n: usize, mut a: [usize; n]}
    a.sort();
    let mut kl: usize = 0;
    let mut kr: usize = n / 2 + 1;
    while (kr - kl) > 1 {
        let k_mid: usize = (kl + kr) / 2;
        if judge(k_mid, &a) {
            kl = k_mid;
        } else {
            kr = k_mid;
        }
    }
    println!("{}", kr - 1);
}

fn judge(k: usize, a: &Vec<usize>) -> bool {
    let n: usize = a.len();
    // check the case of k: 0, 1
    for i in 0..k {
        // both indexes are increasing: two scanners
        if a[i] * 2 > a[n-k+i] {
            return false;
        }
    }
    return true;
}