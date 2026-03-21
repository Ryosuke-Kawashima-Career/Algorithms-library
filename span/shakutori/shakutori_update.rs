use proconio::input;
// 典型76
// 条件を満たす区間: 尺取り, lowerbound
// 区間が循環する: 配列を2倍にする
fn main() {
    input!{n: usize, a: [i64; n]}
    let whole_length: i64 = a.iter().sum();
    if whole_length % 10 != 0 {
        println!("No");
        return;
    }
    let tenth: i64 = whole_length / 10;

    // a[i]からの距離
    let mut length: i64 = 0;
    let mut cur_index: usize = 0;
    for i in 0..n {
        // 1. 条件判定に使う変数を定義
        if length == 0 {
            length += a[i];
        }
        // 2. しゃくとりのindexを初期化
        let mut index: usize = cur_index;
        // 3. 長さが条件をはみ出さないようにする
        while length + a[(index + 1) % n] <= tenth {
            length += a[(index + 1) % n];
            index += 1;
        }

        if length == tenth {
            println!("Yes");
            return;
        }

        // 4. しゃくとりのindexの更新
        if index == i {
            cur_index = index + 1;
        } else {
            cur_index = index;
        }

        // 5. 条件判定用の変数を更新(saturating sub might be better)
        length -= a[i];
    }

    println!("No");
}