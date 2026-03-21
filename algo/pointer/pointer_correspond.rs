use proconio::{input, marker::{Chars, Usize1}};
// abc314C
// pointerで文字の場所を表す
fn main() {
    input!{n: usize, m: usize, s: Chars, c: [Usize1; n]}
    // color_indexes[color: 0..m][index: 0..n]
    let mut color_indexes = vec![vec![]; m];
    for i in 0..n {
        color_indexes[c[i]].push(i);
    }
    let mut pointers: Vec<usize> = (0..n).collect();
    for color in 0..m {
        let color_num = color_indexes[color].len();
        // 更新の際にデータの消滅を防ぐために答えを格納
        let mut next_pointers = Vec::new();
        // 右に巡回シフト: mod上で-1(mod-1)を計算する
        for i in 0..color_num {
            next_pointers.push(pointers[color_indexes[color][(i+color_num-1) % color_num]]);
        }
        for i in 0..color_num {
            pointers[color_indexes[color][i]] = next_pointers[i];
        }
    }
    
    for i in 0..n {
        print!("{}", s[pointers[i]]);
    }
    println!("");
}