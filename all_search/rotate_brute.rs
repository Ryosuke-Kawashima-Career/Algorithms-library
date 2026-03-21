use proconio::input;
use proconio::marker::Chars;
const INF: usize = 1 << 60;
// abc320C
// まずは全探索
// 比較対象が3つあるので三回繰り返す
fn main() {
    input!{m: usize, s1: Chars, s2: Chars, s3: Chars}
    let mut min_time: usize = INF;

    for t1 in 0..3*m {
        for t2 in 0..3*m {
            for t3 in 0..3*m {
                if t1 != t2 && t2 != t3 && t3 != t1 
                && s1[t1 % m] == s2[t2 % m] && s2[t2 % m] == s3[t3 % m] && s3[t3 % m] == s1[t1 % m]
                {
                    let time: usize = t1.max(t2).max(t3);
                    min_time = min_time.min(time);
                }
            }
        }
    }

    if min_time == INF {
        println!("-1");
    } else {
        println!("{}", min_time);
    }
}