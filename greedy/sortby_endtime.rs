use proconio::input;
// 鉄則A39
// 貪欲法とsortは相性がいい
fn main() {
    input!{n: usize, mut lr: [(usize, usize); n]}
    // 終了時間でsort
    lr.sort_by_key(|tup| tup.1);
    let mut cur_time: usize = 0;
    let mut cnt: usize = 0;

    for i in 0..n {
        if cur_time <= lr[i].0 {
            cnt += 1;
            cur_time = lr[i].1;
        }
    }

    println!("{}", cnt);
}