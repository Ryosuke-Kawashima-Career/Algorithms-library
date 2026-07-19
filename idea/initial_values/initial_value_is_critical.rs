use proconio::input;
// 初期値をどのように設定するかが重要!!
// abc339C
fn main() {
    // a[i]: iの駅で電車に乗る人数
    input!{n: usize, a: [i64; n]}
    // ある駅における最小の乗車人数
    // 初期値に注目する: 初めは0人，電車に乗車している
    let mut min_people: i64 = 0;
    let mut cur_people: i64 = 0;
    for i in 0..n {
        cur_people += a[i];
        min_people = min_people.min(cur_people);
    }
    // min_peopleは非負整数!
    let ans: i64 = cur_people + min_people.abs();
    println!("{}", ans);
}