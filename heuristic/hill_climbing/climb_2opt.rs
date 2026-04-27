use proconio::derive_readable;
use proconio::input;
use std::time::{Duration, Instant};
// 鉄則A46
#[derive_readable]
#[derive(Debug, Copy, Clone)]
struct Point {
    x: f64,
    y: f64,
}
impl Point {
    fn new(pos_x: f64, pos_y: f64) -> Self {
        Self { x: pos_x, y: pos_y }
    }
}
// 2-opt法: 区間の移動順序を反転させる
fn main() {
    input! {n: usize, xy: [Point; n]}
    // 開始時間と制限時間
    let start_time = Instant::now();
    let time_limit = Duration::from_millis(990);
    let mut xor = xorshift();

    let mut used: Vec<bool> = vec![false; n];
    let mut cur_v: usize = 0;
    let mut path: Vec<usize> = Vec::new();
    path.push(cur_v);

    // 貪欲法: greedy algorithm
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

    // 局所探索法(山登り法): Hill Climbing(HC)
    let mut cur_length: f64 = calc_total(&path, &xy);
    while start_time.elapsed() < time_limit {
        // [1 n)
        let mut idx_l: usize = range(xor(), 1, n);
        let mut idx_r: usize = range(xor(), 1, n);
        if idx_l > idx_r {
            std::mem::swap(&mut idx_l, &mut idx_r);
        }
        let mut new_path = path.clone();
        // l..=rを反転させる
        new_path[idx_l..=idx_r].reverse();
        let new_length: f64 = calc_total(&new_path, &xy);
        if cur_length >= new_length {
            cur_length = new_length;
            path = new_path;
        }
    }

    for &v in path.iter() {
        println!("{}", v + 1);
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

fn calc_total(path: &Vec<usize>, xy: &Vec<Point>) -> f64 {
    let mut total_dist: f64 = 0.0;
    let n: usize = xy.len();
    for i in 0..n {
        total_dist += calc_dist(path[i], path[i + 1], xy);
    }
    total_dist
}

#[allow(dead_code)]
fn calc_diff(i: usize, j: usize, xy: &Vec<Point>, path: &Vec<usize>) -> f64 {
    let before_dist: f64 = calc_dist(path[i - 1], path[i], xy)
        + calc_dist(path[i], path[i + 1], xy)
        + calc_dist(path[j - 1], path[j], xy)
        + calc_dist(path[j], path[j + 1], xy);
    let after_dist: f64 = calc_dist(path[i - 1], path[j], xy)
        + calc_dist(path[j], path[i + 1], xy)
        + calc_dist(path[j - 1], path[i], xy)
        + calc_dist(path[i], path[j + 1], xy);
    after_dist - before_dist
}

// min..max
#[allow(dead_code)]
fn range(rng_num: u32, min_num: usize, max_num: usize) -> usize {
    min_num + rng_num as usize % (max_num - min_num)
}

#[allow(dead_code)]
fn xorshift() -> Box<dyn FnMut() -> u32> {
    let mut y = 2463534242;
    Box::new(move || {
        y = y ^ (y << 13);
        y = y ^ (y >> 17);
        y = y ^ (y << 5);
        y
    })
}

#[allow(dead_code)]
pub struct Xorshift {
    state: u32,
}

impl Xorshift {
    pub fn new(seed: u32) -> Xorshift {
        Xorshift { state: seed }
    }

    pub fn next(&mut self) -> u32 {
        let mut x: u32 = self.state;
        x ^= x << 13;
        x ^= x >> 17;
        x ^= x << 5;
        self.state = x;
        self.state
    }

    pub fn range(&mut self, min_num: usize, max_num: usize) -> usize {
        min_num + self.next() as usize % (max_num - min_num)
    }
}
