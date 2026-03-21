use proconio::{input, marker::Usize1};
// abc313B
// 推移律: x > y && y > z <=> x > z
// 全順序: ある1,2,…,Nを並べ替えて出来る数列pが存在して、強さがpで番号が登場する順番として表せる
// 自分より強いやつが0人である人がただ一人 <=> そいつが最強
fn main() {
    input!{n: usize, m: usize, edges: [(Usize1, Usize1); m]}
    let mut count_superiors: Vec<usize> = vec![0; n];
    for &(_, y) in edges.iter() {
        count_superiors[y] += 1;
    }

    let mut zero_degrees = 0;
    let mut champion = n;
    for v in 0..n {
        if count_superiors[v] == 0 {
            zero_degrees += 1;
            champion = v;
        }
    }

    // グラフの形が木構造の時，入次数が0のノードは一つだけ
    if zero_degrees == 1 && champion != n {
        println!("{}", champion + 1);
    } else {
        println!("-1");
    }
}