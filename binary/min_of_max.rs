use proconio::input;
// abc023d: 射撃王
// 答えの全探索
// 最大値の最小値を求める
fn main() {
    input!{n: usize, hs: [(usize, usize); n]}

    let mut hl: usize = 0;
    let mut hr: usize = 1 << 60;
    while hr - hl > 1 {
        let mid: usize = (hl + hr) / 2;
        if is_ok(mid, &hs) {
            hr = mid;
        } else {
            hl = mid;
        }
    }

    println!("{}", hr);
}

fn is_ok(h_limit: usize, hs: &Vec<(usize, usize)>) -> bool {
    let n: usize = hs.len();
    let mut remain_times: Vec<usize> = Vec::new();
    for &(height, speed) in hs.iter() {
        if height > h_limit {
            return false;
        }
        remain_times.push((h_limit - height) / speed);
    }
    remain_times.sort();

    for time in 0..n {
        if remain_times[time] < time {
            return false;
        }
    }
    
    return true;
}