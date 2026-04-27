use proconio::input;
use std::time::{Duration, Instant};
const N1: usize = 1usize.wrapping_neg();
const D4: [(usize, usize); 4] = [
    (N1, 0),
    (0, N1), (0, 1),
    (1, 0)
];
const D8: [(usize, usize); 8] = [
    (N1, N1), (N1, 0), (N1, 1),
    (0, N1), (0, 1),
    (1, N1), (1, 0), (1, 1)
];
const N: usize = 100;
const LIMIT: usize = 1000;
// 鉄則A50
fn main() {
    // 開始時間と制限時間
    let start_time = Instant::now();
    let time_limit = Duration::from_millis(5900);
    let mut xor = Xorshift::new(1);

    input!{a: [[i64; N]; N]}
    let mut b: Vec<Vec<i64>> = vec![vec![0; N]; N];
    // (y, x, diff from neighbors)
    let mut diffs: Vec<(usize, usize, f64)> = calc_tilt(&a);
    diffs.sort_by(|x, y| y.2.partial_cmp(&x.2).unwrap());
    let mut operations: Vec<(usize, usize, i64)> = Vec::new();

    // 初期解を作る
    let mut op_cnt: usize = 0;
    for &(y, x, _) in diffs.iter() {
        let h: i64 = a[y][x] - b[y][x];
        if h <= 0 {
            continue;
        }
        let add_num: i64 = (N as i64).min(h);
        for _ in 0..(h / add_num) {
            mount_add(y, x, add_num, &mut b);
            op_cnt += 1;
            // h is 1..=N!
            operations.push((x, y, add_num));
            if evaluate(&a, &b) == 0 {
                print_operations(&operations);
                return;
            }
            if op_cnt == LIMIT {
                break;
            }
        }
    }

    // 山登り法を応用し焼きなまし法を適用する
    let mut cur_diff: i64 = evaluate(&a, &b);
    while start_time.elapsed() < time_limit {
        if cur_diff == 0 {
            break;
        }
        let q: usize = operations.len();
        let idx: usize = xor.range(0, q);
        let (x, y, h) = operations[idx];

        // 変化量を時間経過とともに小さくする
        let time: f64 = start_time.elapsed().as_millis() as f64;
        let dxy: i64 = delta(time, 1, 3);
        let dh: i64 = delta(time, 1, 15);

        let new_x: usize = (x as i64 + xor.rangei(-dxy, dxy)).max(0).min(N as i64 -1) as usize;
        let new_y: usize = (y as i64 + xor.rangei(-dxy, dxy)).max(0).min(N as i64 -1) as usize;
        let new_h: i64 = (h + xor.rangei(-dh, dh)).max(1).min(N as i64);
        // undo
        mount_add(y, x, -h, &mut b);
        // climb
        mount_add(new_y, new_x, new_h, &mut b);
        let new_diff: i64 = evaluate(&a, &b);
        if new_diff == 0 {
            break;
        }

        // 焼きなまし法
        let time: f64 = start_time.elapsed().as_millis() as f64;
        let adoption_rate = adopt_rate(time, cur_diff, new_diff);
        //　山登り法では if cur_diff > new_diff
        if adoption_rate > xor.rangef(0, 1) {
            // update operations
            operations[idx] = (new_x, new_y, new_h);
            cur_diff = new_diff;
        } else {
            // undo climb
            mount_add(new_y, new_x, -new_h, &mut b);
            // restore change
            mount_add(y, x, h, &mut b);
        }
    }

    print_operations(&operations);
}

fn manhattan(y0: usize, x0: usize, y: usize, x: usize) -> i64 {
    let abs_xy: usize = x.abs_diff(x0) + y.abs_diff(y0);
    abs_xy as i64
}

fn evaluate(a: &Vec<Vec<i64>>, b: &Vec<Vec<i64>>) -> i64 {
    let mut score: i64 = 0;
    for i in 0..N {
        for j in 0..N {
            score += (a[i][j] - b[i][j]).abs();
        }
    }
    return score;
}

// 温度は単調減少する
fn temp(time: f64) -> f64 {
    // 5900.0: 全体の時間の長さ
    180.0 - 179.0 * time / 5900.0
}
// 変化量は単調減少する
fn delta(time: f64, min_num: i64, max_num: i64) -> i64 {
    // 5900.0: 全体の時間の長さ
    let diff: f64 = max_num as f64 - min_num as f64 * time / 5900.0;
    diff as i64
}

fn adopt_rate(time: f64, pre: i64, next: i64) -> f64 {
    // 0.0: 改善する場合, pre - next: 悪化する場合 Δ < 0
    let pow = (0.0f64).min((pre - next) as f64 / temp(time));
    pow.exp()
}

fn calc_tilt(a: &Vec<Vec<i64>>) -> Vec<(usize, usize, f64)> {
    let mut res: Vec<(usize, usize, f64)> = Vec::new();
    for y in 0..N {
        for x in 0..N {
            let mut diff_sum: i64 = 0;
            let mut cnt: usize = 0;
            for &(dy, dx) in D8.iter() {
                let next_y: usize = y.wrapping_add(dy);
                let next_x: usize = x.wrapping_add(dx);
                if next_y < N && next_x < N {
                    diff_sum += a[y][x] - a[next_y][next_x];
                    cnt += 1;
                }
            }
            let diff_average: f64 = (diff_sum as f64) / (cnt as f64);
            res.push((y, x, diff_average));
        }
    }
    return res;
}

// 幅優先探索
fn mount_add(y0: usize, x0: usize, h: i64, b: &mut Vec<Vec<i64>>) {
    let mut que = std::collections::VecDeque::new();
    que.push_back((y0, x0));
    let mut seen: Vec<Vec<bool>> = vec![vec![false; N]; N];

    while let Some((y, x)) = que.pop_front() {
        if seen[y][x] {
            continue;
        }
        seen[y][x] = true;
        let dist = manhattan(y0, x0, y, x);
        let cur_h = if h > 0 {
            h - dist
        } else {
            h + dist
        };
        b[y][x] += cur_h;
        if cur_h.abs() <= 1 {
            continue;
        }
        for &(dy, dx) in D4.iter() {
            let next_y: usize = y.wrapping_add(dy);
            let next_x: usize = x.wrapping_add(dx);
            let next_dist = manhattan(y0, x0, next_y, next_x);
            if next_y < N && next_x < N && next_dist > dist && !seen[next_y][next_x] {
                que.push_back((next_y, next_x));
            }
        }
    }
}

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

    pub fn rangei(&mut self, min_num: i64, max_num: i64) -> i64 {
        min_num + self.next() as i64 % (max_num - min_num)
    }

    pub fn rangef(&mut self, min_num: usize, max_num: usize) -> f64 {
        // rng: 0..1
        let rng = (self.next() as f64) / (u32::MAX as f64);
        min_num as f64 + (rng * ((max_num - min_num) as f64))
    }
}

#[allow(dead_code)]
fn print_vec(b: &Vec<Vec<i64>>) {
    for i in 0..N {
        for j in 0..N {
            print!("{} ", b[i][j]);
        }
        println!("");
    }
}

#[allow(dead_code)]
fn print_operations(operations: &Vec<(usize, usize, i64)>) {
    let q: usize = operations.len();
    println!("{}", q);

    for &(y, x, h) in operations.iter() {
        println!("{} {} {}", x, y, h);
    }
}