use proconio::input;
use proconio::derive_readable;
// 典型36
const INF: i64 = 1 << 60;

#[derive_readable]
#[derive(Debug, Copy, Clone)]
struct Point {
    x: i64, y: i64
}

impl Point {
    fn manhattan(x: i64, y: i64) -> Self {
        // マンハッタン距離の計算のために座標を45度回転する。
        Self{x: x-y, y: x+y}
    }
}

fn main() {
    input!{n: usize, q: usize}
    let mut coordinate: Vec<Point> = Vec::new();
    let mut max_x: i64 = -INF;
    let mut max_y: i64 = -INF;
    let mut min_x: i64 = INF;
    let mut min_y: i64 = INF;
    for _ in 0..n {
        input!{x: i64, y: i64}
        let point_rot45 = Point::manhattan(x, y);
        max_x = max_x.max(point_rot45.x);
        max_y = max_y.max(point_rot45.y);
        min_x = min_x.min(point_rot45.x);
        min_y = min_y.min(point_rot45.y);
        coordinate.push(point_rot45);
    }

    // マンハッタン距離を一つの軸上で計算できる。max(dist_x, dist_y)
    for _ in 0..q {
        input!{point: usize}
        let point = point-1;
        let mut max_dist: i64 = 0;
        max_dist = max_dist.max((coordinate[point].x-max_x).abs());
        max_dist = max_dist.max((coordinate[point].y-max_y).abs());
        max_dist = max_dist.max((coordinate[point].x-min_x).abs());
        max_dist = max_dist.max((coordinate[point].y-min_y).abs());

        println!("{}", max_dist);
    }
}