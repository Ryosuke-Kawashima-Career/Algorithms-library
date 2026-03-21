use proconio::{input, marker::Usize1};
// abc327C
// how to express a grid
fn main() {
    input!{a: [[Usize1; 9]; 9]}
    if ok_rows(&a) && ok_cols(&a) && ok_grids(&a) {
        println!("Yes");
    } else {
        println!("No");
    }
}

fn ok_grids(a: &Vec<Vec<usize>>) -> bool {
    let mut is_ok = true;
    // 3x3のグリッドは縦横3つずつ
    for row_grid in 0..3 {
        for col_grid in 0..3 {
            let mut counts: [usize; 9] = [0; 9];
            for i in 0..3 {
                for j in 0..3 {
                    let row: usize = 3 * row_grid + i;
                    let col: usize = 3 * col_grid + j;
                    counts[a[row][col]] += 1;
                }
            }
            if counts != [1; 9] {
                is_ok = false;
            }
        }
    }
    is_ok
}
fn ok_rows(a: &Vec<Vec<usize>>) -> bool {
    let mut is_ok = true;
    for i in 0..9 {
        let mut counts: [usize; 9] = [0; 9];
        for j in 0..9 {
            counts[a[i][j]] += 1;
        }
        if counts != [1; 9] {
            is_ok = false;
        }
    }
    is_ok
}
fn ok_cols(a: &Vec<Vec<usize>>) -> bool {
    let mut is_ok = true;
    for j in 0..9 {
        let mut counts: [usize; 9] = [0; 9];
        for i in 0..9 {
            counts[a[i][j]] += 1;
        }
        if counts != [1; 9] {
            is_ok = false;
        }
    }
    is_ok
}