use proconio::input;
// abc354E: 二次元の上位互換 -> 平面走査(二次元座標にプロットする)
// Q. 2つのカードx,yであって、Ax>AyかつCx<Cyであるようなxを選ぶ。カードyを捨てる。
// A. 二つの軸のうち一つに対してソートする
// A. 二次元座標で自分の右下に点がないカードが答え
fn main() {
    input!{n: usize, ac: [(i64, i64); n]}
    let mut coord_2d: Vec<(i64, i64, usize)> = Vec::new();
    for i in 0..n {
        coord_2d.push((ac[i].0, ac[i].1, i));
    }
    // コストが小さい順にみる
    coord_2d.sort_by_key(|&(|_strengh, cost, _index)| cost);
    let mut max_strength: i64 = 0;
    let mut remains: Vec<usize> = Vec::new();

    // コストが小さいものから探索し，現状で一番強いものが残る
    for &(strengh, cost, index) in coord_2d.iter() {
        if max_strength < strengh {
            max_strength = strengh;
            remains.push(index);
        }
    }

    remains.sort();
    print_answer(&remains);
}

fn print_answer(remains: &Vec<usize>) {
    let m: usize = remains.len();
    println!("{}", m);
    for i in 0..m {
        print!("{} ", remains[i] + 1);
    }
}