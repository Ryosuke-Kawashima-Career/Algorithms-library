use proconio::input;
// GigaCode2019D
fn main() {
    input!{h: usize, w: usize, k: i64, v: i64, a: [[i64; w]; h]}
    let mut prefix: Vec<Vec<i64>> = vec![vec![0; w+1]; h+1];
    for i in 1..=h {
        for j in 1..=w {
            prefix[i][j] = prefix[i][j-1] + a[i-1][j-1];
        }
    }
    for i in 1..=h {
        for j in 1..=w {
            prefix[i][j] += prefix[i-1][j];
        }
    }
    let mut max_area: usize = 0;

    // 区間DPのパクリを2次元で行う
    for span_h in 1..=h {
        for span_w in 1..=w {
            let area: usize = span_h * span_w;
            let house_value: i64 = k * (area as i64);

            for i in 0..=h {
                for j in 0..=w {
                    let end_i: usize = i + span_h;
                    let end_j: usize = j + span_w;
                    if end_i > h || end_j > w {
                        continue;
                    }
                    let area_value: i64 = {
                        prefix[i][j] + prefix[end_i][end_j]
                        -prefix[i][end_j] - prefix[end_i][j]
                    };
                    let total_value: i64 = house_value + area_value;
                    if total_value <= v {
                        max_area = max_area.max(area);
                    }
                }
            }
        }
    }
    println!("{}", max_area);
}