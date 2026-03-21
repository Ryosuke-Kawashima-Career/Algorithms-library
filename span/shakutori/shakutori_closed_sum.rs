use proconio::input;
// 鉄則B13
// しゃくとり法: 条件を満たす区間の大きさを求める
fn main() {
    input!{n: usize, k: i64, a: [i64; n]}
    let mut sum: i64 = 0;
    let mut cur_index: usize = 0;
    let mut ans: usize = 0;
    for i in 0..n {
        let mut index: usize = cur_index;
        // 判定条件sumの条件分岐
        if index == i {
            sum = 0;
        }
        // 条件をはみ出さないようにする: closed
        while index < n && sum + a[index] <= k {
            sum += a[index];
            index += 1;
        }

        ans += index - i;
        // cur_indexの条件分岐
        if index == i {
            cur_index = i + 1;
        } else {
            cur_index = index;
        }
        // 合計値からいらない要素を消す
        sum -= a[i];
    }
    println!("{}", ans);
}