use proconio::input;
// ABC406C
// Q. you are given a permutation of (1,2,...,N). Find the number of (contiguous) subarrays of P that are tilde-shaped. tilde-shape is defined as follows: (~: tilde)
// - The length |A| is at least 4.
// - A1 < A2.
// - There exists exactly one integer i with 2 ≤ i < |A| such that Ai-1 < Ai > Ai+1.
// - There exists exactly one integer i with 2 ≤ i < |A| such that Ai-1 > Ai < Ai+1.
// A. Run-Length Encoding
// A. ~の中央の減少部分に注目する．
fn main() {
    input! {n: usize, p: [usize; n]}
    let mut shifts: Vec<char> = Vec::new();
    for i in 1..n {
        if p[i - 1] < p[i] {
            shifts.push('<');
        } else if p[i - 1] > p[i] {
            shifts.push('>');
        }
    }
    let shifts_rle: Vec<(char, usize)> = rle(&shifts);
    let mut ans: usize = 0;
    for i in 0..shifts_rle.len() {
        let mut left_num: usize = 0;
        let mut right_num: usize = 0;
        if shifts_rle[i].0 == '>' {
            if i >= 1 {
                left_num = shifts_rle[i - 1].1;
            }
            if i < shifts_rle.len() - 1 {
                right_num = shifts_rle[i + 1].1;
            }
            ans += left_num * right_num;
        }
    }
    println!("{}", ans);
}

fn rle<T: PartialEq + Copy>(array: &[T]) -> Vec<(T, usize)> {
    let mut res: Vec<(T, usize)> = Vec::new();
    let n: usize = array.len();
    let mut l: usize = 0;
    while l < n {
        let mut r: usize = l + 1;
        while r < n && array[r] == array[l] {
            r += 1;
        }
        res.push((array[l], r - l));
        l = r;
    }
    return res;
}
