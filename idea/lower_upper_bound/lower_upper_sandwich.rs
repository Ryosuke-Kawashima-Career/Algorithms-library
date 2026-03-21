use proconio::input;
// Q. t=0, F(t=0) = H > 0
// when t = ti, li <= F(t) <= ui
// A. Check the constraints!
// Lowerbound <= F(t) <= Upperbound!!!
// 注目している値がとりうる範囲を挟み撃ちする。
fn main() {
    input! {t: usize}
    for _case in 0..t {
        // for example
        // 2 5
        // 3 1 4
        // 8 9 11
        // = F(0) = 5 => [1 <= F(3) = 4 <= 8] =>  [9 <= F(8) <= 11]
        input! {n: usize, h: i64, tlu: [(i64, i64, i64); n]}
        let mut h_lowerbound: i64 = h;
        let mut h_upperbound: i64 = h;
        let mut cur_time: i64 = 0;
        let mut is_ok: bool = true;
        // sandwich the h_lowerbound and h_upperbound
        for i in 0..n {
            let (time, lowest, highest) = tlu[i];
            let move_time = time - cur_time;
            let h_lowest_possible = 0.max(h_lowerbound - move_time);
            let h_highest_possible = h_upperbound + move_time;
            if h_highest_possible < lowest || h_lowest_possible > highest {
                is_ok = false;
                break;
            }
            h_lowerbound = h_lowest_possible.max(lowest).max(0);
            h_upperbound = h_highest_possible.min(highest);
            cur_time = time;
        }
        if is_ok {
            println!("Yes");
        } else {
            println!("No");
        }
    }
}
