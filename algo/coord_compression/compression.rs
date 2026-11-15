use proconio::input;
use superslice::Ext;
// abc036c
fn main() {
    input!{n: usize, a: [i64; n]}
    // 座標圧縮: 型に気を付ける。
    let comp: Vec<usize> = compress(&a);
    for i in 0..n {
        println!("{}", comp[i]);
    }
}

fn compress(a: &Vec<i64>) -> Vec<usize> {
    let n: usize = a.len();
    let mut b: Vec<i64> = a.clone();
    // 重複する要素を1つにできる。
    b.sort();
    b.dedup();
    let mut res: Vec<usize> = Vec::new();

    for i in 0..n {
        let index: usize = b.lower_bound(&a[i]);
        res.push(index);
    }

    return res;
}

fn compress(a: &Vec<i64>) -> Vec<usize> {
    let mut coord: Vec<usize> = Vec::new();
    let mut values: Vec<i64> = a.clone();
    values.sort();
    values.dedup();
    for value in a.iter() {
        let index: usize = values.partition_point(|x| x < value);
        coord.push(index);
    }
    return coord;
}