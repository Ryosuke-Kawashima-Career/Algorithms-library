use proconio::input;
// 典型66
// 期待値の線形性を利用する
// 和の期待値は期待値の和
fn main() {
    input!{n: usize, lr: [(usize, usize); n]}
    let mut expect: f64 = 0.0;

    // i,j間の期待値を全体の期待値に足す。
    for i in 0..n {
        for j in i+1..n {
            let all_case: usize = (lr[i].1 - lr[i].0 + 1) * (lr[j].1 - lr[j].0 + 1);
            let mut inv_case: usize = 0;

            for num_i in lr[i].0..=lr[i].1 {
                for num_j in lr[j].0..=lr[j].1 {
                    // i < j && num_i > num_j
                    if num_i > num_j {
                        inv_case += 1;
                    }
                }
            }

            expect += (inv_case as f64) / (all_case as f64);
        }
    }

    println!("{}", expect);
}