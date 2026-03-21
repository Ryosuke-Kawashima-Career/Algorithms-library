use proconio::{input, marker::Usize1};
// abc331E
// Heapをつかう貪欲法
// ドント方式に似たアルゴリズム
fn main() {
    input!{n: usize, m: usize, l: usize, mains: [i64; n], b: [i64; m], cd: [(Usize1, Usize1); l]}
    let mut banned = std::collections::HashSet::new();
    for i in 0..l {
        banned.insert(cd[i]);
    }
    let mut subs: Vec<(usize, i64)> = Vec::new();
    for i in 0..m {
        subs.push((i, b[i]));
    }
    // 大きい順に並び替え: 貪欲法
    subs.sort_by(|x, y| y.1.cmp(&x.1));
    // 主菜iが副菜のどのindexまで調べたか?
    let mut cur_main_sub: Vec<usize> = vec![0; n];
    // (主菜と副菜の値段, i: 主菜のindex) 副菜のindexはsubs[cur_main_sub[i]].0
    let mut max_comb = std::collections::BinaryHeap::new();
    for i in 0..n {
        max_comb.push((mains[i] + subs[cur_main_sub[i]].1, i));
    }

    let mut ans: i64 = 0;
    while let Some((price, main_idx)) = max_comb.pop() {
        let sub_idx: usize = subs[cur_main_sub[main_idx]].0;
        if !banned.contains(&(main_idx, sub_idx)) {
            ans = ans.max(price);
            break;
        }
        cur_main_sub[main_idx] += 1;
        if cur_main_sub[main_idx] < m {
            // (定食の値段, 主菜のindex)
            max_comb.push((mains[main_idx]+subs[cur_main_sub[main_idx]].1, main_idx));
        }
    }
    println!("{}", ans);
}