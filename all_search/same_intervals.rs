use proconio::input;
use std::collections::{HashMap, HashSet};
// abc385c
// Q. 1. 選んだビルたちは高さが等しい 2. 選んだビルたちは等間隔に並んでいる
// A. 最大で何個選べるか？
fn main() {
    input!{n: usize, h: [usize; n]}
    let mut ans: usize = 1;
    let mut map = HashMap::new();
    // height: indexes
    for i in 0..n {
        (*map.entry(h[i]).or_insert(vec![])).push(i);
    }
    for (_, vec) in map.iter() {
        let mut set = HashSet::new();
        let n_cur: usize = vec.len();
        for i in 0..n_cur {
            set.insert(vec[i]);
        }
        for gap in 1..n {
            for &index in vec.iter() {
                let mut start = index;
                let mut cnt: usize = 1;
                // Hashsetで数直線を管理する
                while set.contains(&(start+gap)) {
                    cnt += 1;
                    start += gap;
                }
                ans = ans.max(cnt);
            }
        }
    }
    println!("{}", ans);
}