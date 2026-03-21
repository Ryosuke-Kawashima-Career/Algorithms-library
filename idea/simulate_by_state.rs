use proconio::input;
// 鉄則90A44
// データの持ち方を工夫する。
// 状態を表す変数stateなどを用意する。
fn main() {
    input!{n: usize, q: usize}
    let mut a: Vec<usize> = (1..=n).collect();
    let mut is_reversed: bool = false;

    for _ in 0..q {
        input!{query: usize}
        if query == 1 {
            input!{x: usize, y: usize}
            let x = x-1;
            if is_reversed {
                a[n-x-1] = y;
            } else {
                a[x] = y;
            }
        } else if query == 2 {
            if is_reversed {
                is_reversed = false;
            } else {
                is_reversed = true;
            }
        } else {
            input!{x: usize}
            let x = x-1;
            if is_reversed {
                println!("{}", a[n-x-1]);
            } else {
                println!("{}", a[x]);
            }
        }
    }
}