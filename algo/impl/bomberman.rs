use proconio::{input, marker::Usize1};
use std::collections::BTreeSet;
// abc370D
// Q. ボンバーマン
// 座標を(y, x)を格納するsetで表現する
// 二分探索をデータ構造で実現する
// g1: i番目の要素に「i行目に残っている壁の位置を列番号で持った順序付き集合」を持った配列
// g2: j番目の要素に「j列目に残っている壁の位置を行番号で持った順序付き集合」を持った配列
fn main() {
    input!{h: usize, w: usize, nq: usize, queries: [(Usize1, Usize1); nq]}
    let mut remains_row: Vec<BTreeSet<usize>> = vec![BTreeSet::new(); h];
    let mut remains_col: Vec<BTreeSet<usize>> = vec![BTreeSet::new(); w];
    for i in 0..h {
        for j in 0..w {
            remains_row[i].insert(j);
            remains_col[j].insert(i);
        }
    }

    for &(rq, cq) in queries.iter() {
        // rq, cqに爆弾を設置できるか？
        if remains_row[rq].contains(&cq) {
            remains_row[rq].remove(&cq);
            remains_col[cq].remove(&rq);
            continue;
        }

        // 爆弾でマスを消す
        // up
        if let Some(&above) = remains_col[cq].range(..rq).next_back() {
            remains_col[cq].remove(&above);
            remains_row[above].remove(&cq);
        }
        // down
        if let Some(&below) = remains_col[cq].range(rq..).next() {
            remains_col[cq].remove(&below);
            remains_row[below].remove(&cq);
        }
        // left
        if let Some(&left) = remains_row[rq].range(..cq).next_back() {
            remains_row[rq].remove(&left);
            remains_col[left].remove(&rq);
        }
        // right
        if let Some(&right) = remains_row[rq].range(cq..).next() {
            remains_row[rq].remove(&right);
            remains_col[right].remove(&rq);
        }
    }

    let mut ans: usize = 0;
    for i in 0..h {
        ans += remains_row[i].len();
    }
    println!("{}", ans);
}