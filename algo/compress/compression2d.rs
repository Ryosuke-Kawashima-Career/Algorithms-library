use proconio::input;
use superslice::Ext;
// abc213c
// 二次元の座標圧縮
fn main() {
    input!{h: usize, w: usize, n: usize, ij: [(i64, i64); n]}
    let comp2d: Vec<(usize, usize)> = compress2d(&ij);
    for i in 0..n {
        println!("{} {}", comp2d[i].0+1, comp2d[i].1+1);
    }
}

fn compress2d(a: &Vec<(i64, i64)>) -> Vec<(usize, usize)> {
    let n: usize = a.len();
    // それぞれの軸に対して座標圧縮する。
    let mut x: Vec<i64> = a.iter().map(|&(_, x)| x).collect();
    let mut y: Vec<i64> = a.iter().map(|&(y, _)| y).collect();
    x.sort();
    x.dedup();
    y.sort();
    y.dedup();
    let mut res: Vec<(usize, usize)> = Vec::new();

    for i in 0..n {
        let index_y: usize = y.lower_bound(&a[i].0);
        let index_x: usize = x.lower_bound(&a[i].1);
        res.push((index_y, index_x));
    }

    return res;
}

fn compress2d(a: &Vec<(i64, i64)>) -> Vec<(usize, usize)> {
    let n: usize = a.len();
    // それぞれの軸に対して座標圧縮する。
    let mut x: Vec<i64> = a.iter().map(|&(_, x)| x).collect();
    let mut y: Vec<i64> = a.iter().map(|&(y, _)| y).collect();
    x.sort();
    x.dedup();
    y.sort();
    y.dedup();
    let mut res: Vec<(usize, usize)> = Vec::new();

    for i in 0..n {
        let index_y: usize = y.partition_point(|&yx| yx < a[i].0);
        let index_x: usize = x.partition_point(|&yx| yx < a[i].1);
        res.push((index_y, index_x));
    }

    return res;
}