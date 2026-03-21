use proconio::input;
const LENGTH: usize = 2000;
// ABC434 D
// Q. Which are the areas covered by exactly one rectangle?
// A.1. Use imos to find the areas not covered by any rectangle.
// A.2. Add k for Rectangle k, and use imos to find the areas covered by exactly one rectangle.
// 足し算の値を雲の固有の値にすることで、1つの長方形にだけ覆われているエリアを特定する。
fn main() {
    input! {n: usize, udlr: [(usize, usize, usize, usize); n]}
    let (imos, imos_unique) = normal_and_unique_imos(&udlr);
    let covered_by: Vec<usize> = get_covered_by_clouds(n, &imos, &imos_unique);
    for k in 1..=n {
        let ans = covered_by[0] + covered_by[k];
        println!("{}", ans);
    }
}

fn normal_and_unique_imos(udlr: &Vec<(usize, usize, usize, usize)>) -> (Vec<Vec<i64>>, Vec<Vec<i64>>) {
    let n: usize = udlr.len();
    let mut imos: Vec<Vec<i64>> = vec![vec![0; LENGTH + 2]; LENGTH + 2];
    let mut imos_unique: Vec<Vec<i64>> = vec![vec![0; LENGTH + 2]; LENGTH + 2];
    for k in 1..=n {
        let (u, d, l, r) = udlr[k-1];
        imos[u][l] += 1;
        imos[u][r + 1] -= 1;
        imos[d + 1][l] -= 1;
        imos[d + 1][r + 1] += 1;
        imos_unique[u][l] += k as i64;
        imos_unique[u][r + 1] -= k as i64;
        imos_unique[d+1][l] -= k as i64;
        imos_unique[d+1][r + 1] += k as i64;
    }
    for i in 0..LENGTH + 2 {
        for j in 0..LENGTH + 2 {
            if j == 0 { continue; }
            imos[i][j] += imos[i][j - 1];
            imos_unique[i][j] += imos_unique[i][j - 1];
        }
    }
    for i in 0..LENGTH + 2 {
        for j in 0..LENGTH + 2 {
            if i == 0 { continue; }
            imos[i][j] += imos[i - 1][j];
            imos_unique[i][j] += imos_unique[i-1][j];
        }
    }
    (imos, imos_unique)
}

fn get_covered_by_clouds(clouds: usize, imos: &Vec<Vec<i64>>, unique_imos: &Vec<Vec<i64>>) -> Vec<usize> {
    let mut coverd_by: Vec<usize> = vec![0; clouds+1];
    for i in 1..=LENGTH {
        for j in 1..=LENGTH {
            if imos[i][j] == 0 {
                coverd_by[0] += 1;
            } else if imos[i][j] == 1 {
                /* If covered only by one, you can get the cloud that covers by its unique value */
                let k = unique_imos[i][j] as usize;
                coverd_by[k] += 1;
            }
        }
    }
    return coverd_by;
}