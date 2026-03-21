use proconio::input;
// arc022b
fn main() {
    input!{n: usize, a: [usize; n]}
    let mut ans: usize = 0;
    let mut cur_index: usize = 0;
    // is_used[a[i]]
    let mut is_used: Vec<bool> = vec![false; 100001];

    for i in 0..n {
        let mut index: usize = cur_index;

        while index < n && !is_used[a[index]] {
            is_used[a[index]] = true;
            index += 1;
        }

        ans = ans.max(index-i);
        // 元に戻す。
        is_used[a[i]] = false;
        // indexの更新の仕方で場合分け
        if index == i {
            cur_index = index + 1;
        } else {
            cur_index = index;
        }
    }

    println!("{}", ans);
}