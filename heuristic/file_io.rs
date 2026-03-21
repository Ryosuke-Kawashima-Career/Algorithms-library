use std::{fs::File, io::{Read, BufReader, Write, BufWriter, Result}};
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
    // buf is faster
    let a = read_file("./input/input.txt");
    let a = bufread_file("./input/input.txt");

    let mut b: Vec<Vec<i64>> = vec![vec![0; N]; N];
    // (y, x, diff from neighbors)
    let mut diffs: Vec<(usize, usize, f64)> = calc_tilt(&a);
    diffs.sort_by(|x, y| y.2.partial_cmp(&x.2).unwrap());
    let mut operations: Vec<(usize, usize, i64)> = Vec::new();

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
                break;
            }
            if op_cnt == LIMIT {
                break;
            }
        }
    }

    write_file("./myout.txt", &operations);
    // return Result<()>
    bufwrite_file("./bufout.txt", &operations);
}

fn manhattan(y0: usize, x0: usize, y: usize, x: usize) -> i64 {
    let abs_xy: usize = (x.abs_diff(x0)).wrapping_add(y.abs_diff(y0));
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

// check read to string!
fn read_file(file_name: &str) -> Vec<Vec<i64>> {
    let mut a: Vec<Vec<i64>> = vec![vec![0; N]; N];
    let mut f: File = File::open(file_name).expect("Fail to open file!");
    let mut contents: String = String::new();
    f.read_to_string(&mut contents).expect("Fail to read file!");
    
    let mut lines = contents.lines();
    let mut cur_line: usize = 0;
    while let Some(line) = lines.next() {
        let mut nums = line.split_whitespace();
        let mut cur_col: usize = 0;
        while let Some(num) = nums.next() {
            let num = num.parse().unwrap();
            a[cur_line][cur_col] = num;
            cur_col += 1;
        }
        cur_line += 1;
    }

    return a;
}
fn bufread_file(file_name: &str) -> Vec<Vec<i64>> {
    let mut a: Vec<Vec<i64>> = vec![vec![0; N]; N];
    let mut f = BufReader::new(File::open(file_name).unwrap());
    let mut contents: String = String::new();
    f.read_to_string(&mut contents).expect("Fail to read file!");
    
    let mut lines = contents.lines();
    let mut cur_line: usize = 0;
    while let Some(line) = lines.next() {
        let mut nums = line.split_whitespace();
        let mut cur_col: usize = 0;
        while let Some(num) = nums.next() {
            let num = num.parse().unwrap();
            a[cur_line][cur_col] = num;
            cur_col += 1;
        }
        cur_line += 1;
    }

    return a;
}

fn write_file(file_name: &str, operations: &Vec<(usize, usize, i64)>) {
    let mut f = File::create(file_name).expect("Fail to create a file!");
    let q: usize = operations.len();
    f.write_all(&format!("{}\n", q).as_bytes()).expect("Fail to write!");

    for &(y, x, h) in operations.iter() {
        f.write_all(&format!("{} {} {}\n", x, y, h).as_bytes()).expect("Fail to write!");
    }
}
fn bufwrite_file(file_name: &str, operations: &Vec<(usize, usize, i64)>) -> Result<()> {
    let mut f = BufWriter::new(File::create(file_name).unwrap());
    let q: usize = operations.len();
    f.write_all(&format!("{}\n", q).as_bytes())?;
    
    for &(y, x, h) in operations.iter() {
        f.write_all(&format!("{} {} {}\n", x, y, h).as_bytes())?;
    }

    Ok(())
}

fn print_vec(b: &Vec<Vec<i64>>) {
    for i in 0..N {
        for j in 0..N {
            print!("{} ", b[i][j]);
        }
        println!("");
    }
}

fn print_operations(operations: &Vec<(usize, usize, i64)>) {
    let q: usize = operations.len();
    println!("{}", q);

    for &(y, x, h) in operations.iter() {
        println!("{} {} {}", x, y, h);
    }
}