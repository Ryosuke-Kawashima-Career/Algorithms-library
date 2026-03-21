use proconio::{input, marker::Chars};
use std::collections::HashMap;
// abc315D
// マス目の種類ごとに分類し，それらの個数を記録する
fn main() {
    input!{h: usize, w: usize, c: [Chars; h]}
    // 行と列ごとに文字の数を計算する
    let mut count_rows: Vec<HashMap<u8, usize>> = vec![HashMap::new(); h];
    let mut count_cols: Vec<HashMap<u8, usize>> = vec![HashMap::new(); w];
    for i in 0..h {
        for j in 0..w {
            *count_rows[i].entry(c[i][j] as u8 - 'a' as u8).or_insert(0) += 1;
            *count_cols[j].entry(c[i][j] as u8 - 'a' as u8).or_insert(0) += 1;
        }
    }

    while can_earse(&mut count_rows, &mut count_cols) {}
    let remain_rows: usize = calc_remain(&count_rows);
    let remain_cols: usize = calc_remain(&count_cols);
    println!("{}", remain_rows * remain_cols);
}

fn can_earse(count_rows: &mut Vec<HashMap<u8, usize>>, count_cols: &mut Vec<HashMap<u8, usize>>
) -> bool {
    let mut is_erased = false;
    let h: usize = count_rows.len();
    let w: usize = count_cols.len();

    // 消される色を求める
    let mut erased_colors_row = Vec::new();
    for row in 0..h {
        // 1色が2個以上残っているとき
        if count_rows[row].len() == 1 && *count_rows[row].iter().next().unwrap().1 >= 2 {
            let color = *count_rows[row].iter().next().unwrap().0;
            erased_colors_row.push(color);
            count_rows[row].remove(&color);
            is_erased = true;
        }
    }
    let mut erased_colors_col = Vec::new();
    for col in 0..w {
        // 1色が2個以上残っているとき
        if count_cols[col].len() == 1 && *count_cols[col].iter().next().unwrap().1 >= 2 {
            let color = *count_cols[col].iter().next().unwrap().0;
            erased_colors_col.push(color);
            count_cols[col].remove(&color);
            is_erased = true;
        }
    }

    // 行から消される列の色を消去する
    for row in 0..h {
        for &color in erased_colors_col.iter() {
            if let Some(num) = count_rows[row].get_mut(&color) {
                if *num == 1 {
                    count_rows[row].remove(&color);
                } else {
                    *num -= 1;
                }
            }
        }
    }
    for col in 0..w {
        for &color in erased_colors_row.iter() {
            if let Some(num) = count_cols[col].get_mut(&color) {
                if *num == 1 {
                    count_cols[col].remove(&color);
                } else {
                    *num -= 1;
                }
            }
        }
    }

    return is_erased;
}

fn calc_remain(count: &Vec<HashMap<u8, usize>>) -> usize {
    let n: usize = count.len();
    let mut cnt: usize = 0;
    for i in 0..n {
        if count[i].len() > 0 {
            cnt += 1;
        }
    }
    return cnt;
}