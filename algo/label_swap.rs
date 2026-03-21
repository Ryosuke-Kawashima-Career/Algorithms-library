use proconio::{input, marker::Usize1};
// abc395d
// Q. 仮想的な「ラベル鳩」を考え、次のように問題を読み替えます。
// N 個の巣があり、はじめ i 番目の巣には鳩 i とラベル鳩 i が入っている。
// 次の操作を処理せよ。
// 1. 鳩 i を、ラベル鳩 j が入っている巣に移動させる。
// 2. ラベル鳩 i とラベル鳩 j を交換する。
// 3. 鳩 i と同じ巣に入っているラベル鳩の番号を出力する。
// A. 巣をlabelで代用する
fn main() {
    input!{n: usize, q: usize}
    let mut pigeon_to_nest: Vec<usize> = (0..n).collect();
    let mut label_to_nest: Vec<usize> = (0..n).collect();
    let mut nest_to_label: Vec<usize> = (0..n).collect();
    // 箱の交換をlabelで代替する
    for _ in 0..q {
        input!{op: usize}
        if op == 1 {
            input!{pigeon: Usize1, label: Usize1}
            pigeon_to_nest[pigeon] = label_to_nest[label];
        } else if op == 2 {
            // switch the pigeons on nest_a and nest_b
            input!{label_a: Usize1, label_b: Usize1}
            // switch nest
            let nest_a: usize = label_to_nest[label_a];
            label_to_nest[label_a] = label_to_nest[label_b];
            label_to_nest[label_b] = nest_a;
            // switch label
            let label1: usize = nest_to_label[label_to_nest[label_a]];
            nest_to_label[label_to_nest[label_a]] = nest_to_label[label_to_nest[label_b]];
            nest_to_label[label_to_nest[label_b]] = label1;
        } else {
            input!{pigeon: Usize1}
            let ans_label = nest_to_label[pigeon_to_nest[pigeon]];
            println!("{}", ans_label + 1);
        }
    }
}