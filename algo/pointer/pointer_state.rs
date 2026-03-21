use proconio::input;
// 状態stateを記録してクエリを処理する。
// 典型44
fn main() {
    // a[pointers[shift_x]]
    input!{n: usize, q: usize, a: [i64; n]}
    let mut pointers: Vec<usize> = (0..n).collect();
    let mut shift_state: usize = 0;

    for _ in 0..q {
        input!{t: usize, x: usize, y: usize}
        let (x, y) = (x-1, y-1);

        if t == 1 {
            // switch (do not forget shift!!!)
            // pointerを動かす。
            let shift_x: usize = (x + shift_state) % n;
            let shift_y: usize = (y + shift_state) % n;
            let copy_x: usize = pointers[shift_x];
            pointers[shift_x] = pointers[shift_y];
            pointers[shift_y] = copy_x;
        } else if t == 2 {
            // 右シフト1回: +(n-1), 左シフト1回: +1
            shift_state += n-1;
            // 一周(mod)
            shift_state %= n;
        } else {
            let shift_x: usize = (x + shift_state) % n;
            println!("{}", a[pointers[shift_x]]);
        }
    }
}