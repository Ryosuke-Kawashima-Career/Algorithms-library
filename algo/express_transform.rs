use proconio::input;
// abc417C
// Q. Find how many pairs of integers (i,j) (1≤i<j≤N) satisfy j−i=Ai+Aj.
// A. Do experiments!!!: j−i=Ai+Ajを式変形して、j−Aj=i+Ai 
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