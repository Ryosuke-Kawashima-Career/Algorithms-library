use proconio::input;
// abc357
// 形の再帰構造: 再帰関数で定義
fn main() {
    input!{n: usize}
    let mut carpets: Vec<Vec<Vec<char>>> = Vec::new();
    for level in 0..=n {
        let length: usize = 3usize.pow(level as u32);
        let carpet: Vec<Vec<char>> = vec![vec!['#'; length]; length];
        carpets.push(carpet); 
    }
    solve(n, &mut carpets);
    let length_n: usize = 3usize.pow(n as u32);
    for i in 0..length_n {
        for j in 0..length_n {
            print!("{}", carpets[n][i][j]);
        }
        println!("");
    }
}

fn solve(level: usize, carpets: &mut Vec<Vec<Vec<char>>>) {
    if level == 0 {
        return;
    }
    solve(level - 1, carpets);
    let length: usize = 3usize.pow(level as u32 - 1);

    // row, col: levelにおけるブロック
    for row in 0..3 {
        for col in 0..3 {
            // i, j: level-1におけるy, x
            for i in 0..length {
                for j in 0..length {
                    let y: usize = length * row + i;
                    let x: usize = length * col + j;
                    if row == 1 && col == 1 {
                        carpets[level][y][x] = '.';
                    } else {
                        carpets[level][y][x] = carpets[level-1][i][j];
                    }
                }
            }
        }
    }
}