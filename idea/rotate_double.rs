use proconio::input;
use superslice::Ext;
// 典型76
// 条件を満たす区間: 尺取り, lowerbound
// 区間が循環する: 配列を2倍にする
fn main() {
    input!{n: usize, a: [i64; n]}
    let whole_length: i64 = a.iter().sum();
    let mut lengths_from_start: Vec<i64> = Vec::new();
    lengths_from_start.push(0);
    let mut length_from_start: i64 = 0;

    // 配列を2周することで循環を表す
    for i in 0..(2*n) {
        length_from_start += a[i % n];
        lengths_from_start.push(length_from_start);
    }

    if whole_length % 10 != 0 {
        println!("No");
        return;
    }

    let tenth: i64 = whole_length / 10;
    // startをすべて試す
    for i in 0..n {
        let index: usize = lengths_from_start.lower_bound(&(lengths_from_start[i] + tenth));
        if lengths_from_start[index] == lengths_from_start[i] + tenth {
            println!("Yes");
            return;
        }
    }

    println!("No");
}