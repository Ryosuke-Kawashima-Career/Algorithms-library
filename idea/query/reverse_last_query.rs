use proconio::{input, marker::Usize1};
const MAX: usize = 200_000;
// abc346E
// 0000   0000   0001   0001   0001
// 0000 ->5555 ->5551 ->5551 ->5551 
// 0000   0000   0001   3333   2222
// クエリの反転: 最後のクエリが最後の状態(the last query is the last state)
// 後の操作が以前の結果を塗りつぶすので後ろから確定すればいい
fn main() {
    input!{h: usize, w: usize, m: usize, query: [(usize, Usize1, usize); m]}
    // 塗りつぶす際にいくつのマスを塗れるのか残り
    let mut remain_row = h;
    let mut remain_col = w;
    // マス目の色が確定したか?
    let mut used_row = vec![false; h];
    let mut used_col = vec![false; w];
    // マス目の色がcolorのマスの個数を保存 color_cnt[color]
    let mut color_cnt: Vec<usize> = vec![0; MAX+1];

    // クエリの反転
    for idx in (0..m).rev() {
        let (query_type, a, x) = query[idx];
        // 行を塗りつぶすとき，残りの列の個数分のマスが確定する
        if query_type == 1 { // 行
            if used_row[a] {
                continue;
            }
            color_cnt[x] += remain_col;
            remain_row = remain_row.saturating_sub(1);
            used_row[a] = true;
        } else { // 列
            if used_col[a] {
                continue;
            }
            color_cnt[x] += remain_row;
            remain_col = remain_col.saturating_sub(1);
            used_col[a] = true;
        }
    }

    // 初期状態のままのマスの個数を計算する
    let mut changed_squares = 0;
    for color in 0..=MAX {
        if color_cnt[color] > 0 {
            changed_squares += color_cnt[color];
        }
    }
    // 色0が初期値
    color_cnt[0] += (h * w).saturating_sub(changed_squares);

    // 最後に残った色の個数
    let mut remain_colors: usize = 0;
    for color in 0..=MAX {
        if color_cnt[color] > 0 {
            changed_squares += color_cnt[color];
            remain_colors += 1;
        }
    }
    println!("{}", remain_colors);

    for color in 0..=MAX {
        if color_cnt[color] > 0 {
            println!("{} {}", color, color_cnt[color]);
        }
    }
}