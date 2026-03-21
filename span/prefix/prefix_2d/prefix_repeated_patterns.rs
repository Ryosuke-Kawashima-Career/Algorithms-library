use proconio::input;
use proconio::marker::Chars;
// abc311d
// 繰り返しのバターンの累積和
// 符号の+-に注意する!!!!!!!!!!!!!!!!!
fn main() {
    input!{n: usize, q: usize, p: [Chars; n]}
    let mut prefix: Vec<Vec<usize>> = vec![vec![0; n+1]; n+1];
    // 二次元累積和を取る
    for i in 1..=n {
        for j in 1..=n {
            if p[i-1][j-1] == 'B' {
                prefix[i][j] += prefix[i][j-1] + 1;
            } else {
                prefix[i][j] += prefix[i][j-1];
            }
        }
    }
    for i in 1..=n {
        for j in 1..=n {
            prefix[i][j] += prefix[i-1][j];
        }
    }

    for _ in 0..q {
        input!{a: usize, b: usize, c: usize, d: usize}
        let ab: usize = get_block(a, b, &prefix);
        let cd: usize = get_block(c+1, d+1, &prefix);
        let ad: usize = get_block(a, d+1, &prefix);
        let cb: usize = get_block(c+1, b, &prefix);
        let blocks: usize = ab + cd - ad - cb;
        println!("{}", blocks);
    }
}

fn get_block(i: usize, j: usize, prefix: &Vec<Vec<usize>>) -> usize {
    let mut res: usize = 0;
    // パターンの大きさ
    let n: usize = prefix.len() - 1;
    // パターンに含まれるブロックの数
    let all: usize = prefix[n][n];
    // ブロックが行の向きに何個ならんでいるか？
    let rows: usize = i / n;
    // ブロックが列の向きに何個ならんでいるか？
    let cols: usize = j / n;
    // 範囲内にパターンがいくつあるか？
    res += all * rows * cols;

    // パターンの一部が中途半端に残ったとき
    let remain_row: usize = i % n;
    let remain_col: usize = j % n;
    res += prefix[remain_row][n] * cols;
    res += prefix[n][remain_col] * rows;
    // 符号に注意する!!!
    res += prefix[remain_row][remain_col];

    return res;
}