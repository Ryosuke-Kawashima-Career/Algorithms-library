use proconio::input;
use proconio::derive_readable;
// 鉄則A46
#[derive_readable]
#[derive(Debug, Copy, Clone)]
struct Point {
    x: f64, y: f64
}
impl Point {
    fn new(pos_x: f64, pos_y: f64) -> Self {
        Self{x: pos_x, y: pos_y}
    }
}
// greedy
fn main() {
    input!{n: usize, xy: [Point; n]}
    let mut used: Vec<bool> = vec![false; n];
    let mut cur_v: usize = 0;
    let mut path: Vec<usize> = Vec::new();
    path.push(cur_v);

    // 未来のことも考える: 2つ先の距離まで考える
    let future: usize = 2;
    while !used[0] {
        let mut min_dist: f64 = f64::MAX;
        let mut nearest_v: usize = n;
        // avoid the start 0
        let (dist, next_v) = dfs_future(cur_v, future, &xy, &mut used);
        if min_dist > dist {
            min_dist = dist;
            nearest_v = next_v;
        }
        
        // if only 0 remains
        if nearest_v == n {
            nearest_v = 0;
        }
        used[nearest_v] = true;
        path.push(nearest_v);
        cur_v = nearest_v;
    }

    for &v in path.iter() {
        println!("{}", v+1);
    }
}

// なるべく関数で実装する(クロージャーでは所有権の問題が発生する)
fn calc_dist(i: usize, j: usize, xy: &Vec<Point>) -> f64 {
    let x_sq = (xy[j].x - xy[i].x) * (xy[j].x - xy[i].x);
    let y_sq = (xy[j].y - xy[i].y) * (xy[j].y - xy[i].y);
    (x_sq + y_sq).sqrt()
}

// (distance, nearest vertex)
fn dfs_future(cur_v: usize, future: usize, xy: &Vec<Point>, seen: &mut Vec<bool>) -> (f64, usize) {
    let mut length: f64 = f64::MAX;
    let n: usize = xy.len();
    let mut nearest_v: usize = n;
    // base case
    if future == 0 {
        return (0.0, cur_v);
    }
    // avoid the start 0
    for next_v in 1..n {
        if cur_v == next_v || seen[next_v] {
            continue;
        }
        let next_dist: f64 = calc_dist(cur_v, next_v, xy);
        // 行きがけに次のノードを訪れたことを記録する
        seen[next_v] = true;
        let future_dist: f64 = dfs_future(next_v, future - 1, xy, seen).0;
        // 帰りがけに復元する
        seen[next_v] = false;
        if length > next_dist + future_dist {
            length = next_dist + future_dist;
            nearest_v = next_v;
        }
    }

    if nearest_v == n {
        return (calc_dist(cur_v, 0, xy), 0);
    }

    (length, nearest_v)
}