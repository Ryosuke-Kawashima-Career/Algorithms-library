use proconio::input;
// 典型63
// h<=8の制約に目を付ける -> オーダーが小さい変数に対して全探索
fn main() {
    input!{h: usize, w: usize, p: [[usize; w]; h]}
    let bits: usize = 1 << h;
    let mut max_size: usize = 0;

    for bit in 0..bits {
        // 同じ数の列の個数(val)をその数(key)ごとに記録する.
        let mut count = std::collections::HashMap::new();
        // 使う行を決定する
        let mut rows: Vec<usize> = Vec::new();
        for i in 0..h {
            if bit >> i & 1 == 1 {
                rows.push(i);
            }
        }
        if rows.is_empty() {
            continue;
        }

        // 列の値がすべて同じか判定する
        let mut max_col: usize = 0;
        for j in 0..w {
            let num: usize = p[rows[0]][j];
            let mut is_all_same: bool = true;
            for &row in rows.iter() {
                if p[row][j] != num {
                    is_all_same = false;
                    break;
                }
            }
            
            // 値が更新されるときに最大に挑む挑戦権が与えられる。
            if is_all_same {
                *count.entry(num).or_insert(0) += 1;
                max_col = max_col.max(count[&num]);
            }
        }

        max_size = max_size.max(max_col * rows.len());
    }

    println!("{}", max_size);
}