use proconio::input;
// 典型79
// 操作順序に依らない。
// 1つのマスごとに独立して着目すればいい。
fn main() {
    input!{h: usize, w: usize, mut a: [[i64; w]; h], b: [[i64; w]; h]}
    let mut operations: i64 = 0;
    for i in 0..h-1 {
        for j in 0..w-1 {
            let dif: i64 = b[i][j] - a[i][j];
            for dy in 0..2 {
                for dx in 0..2 {
                    a[i+dy][j+dx] += dif;
                    operations += dif.abs();
                }
            }
        }
    }
    // 4マスの値を増やすのが１回
    operations /= 4;

    if a == b {
        println!("Yes");
        println!("{}", operations);
    } else {
        println!("No");
    }
}