use proconio::input;
const SIZE: usize = 5000;
// 典型81
// 累積和を使用する(imos法の応用)
// 最小値の全探索をする。全探索の対象を工夫する。
fn main() {
    input!{n: usize, k: usize}
    // prefix_2d: 身長aと体重bを二次元座標にplotして生徒数の累積和を出す。
    // (a, b)以上の数を求める
    let mut prefix_2d: Vec<Vec<usize>> = vec![vec![0; SIZE+2]; SIZE+2];
    for _ in 0..n {
        input!{a: usize, b: usize}
        prefix_2d[a][b] += 1;
    }
    // (a, b)以下の場合は
    // 身長と体重がそれぞれab[i].0とab[i].1以下の人の数
    let mut imos: Vec<Vec<i64>> = vec![vec![0; MAX+2]; MAX+2];
    for i in 0..n {
        imos[0][0] += 1;
        imos[ab[i].0 + 1][ab[i].1 + 1] += 1;
        imos[ab[i].0 + 1][0] -= 1;
        imos[0][ab[i].1 + 1] -= 1;
    }

    for i in 1..=SIZE {
        for j in 1..=SIZE {
            prefix_2d[i][j] += prefix_2d[i][j-1];
        }
    }
    for i in 1..=SIZE {
        for j in 1..=SIZE {
            prefix_2d[i][j] += prefix_2d[i-1][j];
        }
    }

    let mut ans: usize = 0;
    // 最小の身長a, 体重bの全探索を行う。
    for min_a in 1..=SIZE {
        for min_b in 1..=SIZE {
            // let max_a = (min_a + k).min(SIZE);でもよい(ifはいらない)
            let max_a: usize = min_a + k;
            let max_b: usize = min_b + k;
            if max_a <= SIZE && max_b <= SIZE {
                let member_num: usize = {
                    prefix_2d[max_a][max_b] + prefix_2d[min_a-1][min_b-1]
                    - prefix_2d[max_a][min_b-1] - prefix_2d[min_a-1][max_b]
                };
                ans = ans.max(member_num);
            }
        }
    }

    println!("{}", ans);
}