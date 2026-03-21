use proconio::input;
// ABC417C
// Q. j−i=Ai+Ajを変形して、j−Aj=i+Ai
// A. Gather the same variable in the same equation
// A. count the left and right independently
fn main() {
    input!{n: usize, a: [usize; n]}
    let mut cnt_left: Vec<usize> = vec![0; n];
    let mut cnt_right: Vec<usize> = vec![0; n];
    for i in 0..n {
        let index_left: isize = i as isize - a[i] as isize;
        let index_right: isize = i as isize + a[i] as isize;
        if index_left >= 0 && index_left < n as isize {
            cnt_left[index_left as usize] += 1;
        }
        if index_right >= 0 && index_right < n as isize {
            cnt_right[index_right as usize] += 1;
        }
    }
    let mut ans: usize = 0;
    for i in 0..n {
        ans += cnt_left[i] * cnt_right[i];
    }
    println!("{}", ans);
}