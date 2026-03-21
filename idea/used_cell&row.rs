use proconio::{input, marker::Usize1};
// ABC406D
// Q. On a 2 dimensional table, points on a specific row or column are disposed of. How is it possible to caluculate queries in unit of rows and columns?
// A. check whether being used not only for each point but also each row and column.
fn main() {
    input!{h: usize, w: usize, n: usize, xy: [(Usize1, Usize1); n], q: usize}
    let mut points_x: Vec<Vec<usize>> = vec![vec![]; h];
    let mut points_y: Vec<Vec<usize>> = vec![vec![]; w];
    let mut counts_x: Vec<usize> = vec![0; h];
    let mut counts_y: Vec<usize> = vec![0; w];
    let mut used: Vec<bool> = vec![false; n];
    let mut used_x: Vec<bool> = vec![false; h];
    let mut used_y: Vec<bool> = vec![false; w];
    for point in 0..n {
        let (x, y) = xy[point];
        points_y[y].push(point);
        points_x[x].push(point);
        counts_y[y] += 1;
        counts_x[x] += 1;
    }
    for _ in 0..q {
        input!{query_type: usize}
        let mut ans: usize = 0;
        if query_type == 1 {
            input!{x: Usize1}
            if used_x[x] {
                println!("{}", ans);
                continue;
            }
            ans = counts_x[x];
            counts_x[x] = 0;
            for &point in points_x[x].iter() {
                if !used[point] {
                    counts_y[xy[point].1] -= 1;
                    used[point] = true;
                }
            }
            used_x[x] = true;
        } else {
            input!{y: Usize1}
            if used_y[y] {
                println!("{}", ans);
                continue;
            }
            ans = counts_y[y];
            counts_y[y] = 0;
            for &point in points_y[y].iter() {
                if !used[point] {
                    counts_x[xy[point].0] -= 1;
                    used[point] = true;
                }
            }
            used_y[y] = true;
        }
        println!("{}", ans);
    }
}