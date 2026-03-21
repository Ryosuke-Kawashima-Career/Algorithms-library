use proconio::input;
// abc326c
// 尺取り法を使う。
fn main() {
    input!{n: usize, m: usize, mut a: [usize; n]}
    a.sort();
    let mut cur_index: usize = 0;
    let mut ans: usize = 0;
    for i in 0..n {
        let mut index: usize = cur_index;
        while index < n && a[index] < a[i] + m {
            index += 1;
        }
        let mut num: usize = index - i;
        ans = ans.max(num);
        cur_index = index;
    }

    println!("{}", ans);
}