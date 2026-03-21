use proconio::input;
// 鉄則A09
fn main() {
    input!{h: usize, w: usize, n: usize}
    // h, w + 2!!!
    let mut prefix_sums: Vec<Vec<i64>> = vec![vec![0; w+2]; h+2];
    // calc layers by 4 vertexes
    for _ in 0..n {
        // y: a,c x: b,d 
        input!{a: usize, b: usize, c: usize, d: usize}
        prefix_sums[a][b] += 1;
        prefix_sums[c+1][d+1] += 1;
        prefix_sums[a][d+1] -= 1;
        prefix_sums[c+1][b] -= 1;
    }

    for i in 1..=h {
        for j in 1..=w {
            // 横方向に累積和を取る。
            prefix_sums[i][j] += prefix_sums[i][j-1];
        }
    }
    for i in 1..=h {
        for j in 1..=w {
            // 縦方向に累積和を取る。
            prefix_sums[i][j] += prefix_sums[i-1][j];
        }
    }

    for i in 1..=h {
        for j in 1..=w {
            print!("{} ", prefix_sums[i][j]);
        }
        println!("");
    }
}