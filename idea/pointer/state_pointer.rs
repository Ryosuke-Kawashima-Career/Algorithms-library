use proconio::input;
// 鉄則B45
// 状態を保持する．
// indexの遷移をポインターで管理する。
fn main() {
    input!{n: usize, a: [[usize; n]; n], q: usize}
    // interface -> actual value
    let mut pointers: Vec<usize> = (0..n).collect();
    for _ in 0..q {
        input!{query: usize, x: usize, y: usize}
        // interface(x, y) -> actual values(pointers[x, y])
        let (x, y) = (x-1, y-1);
        if query == 1 {
            let pointer_y: usize = pointers[y];
            pointers[y] = pointers[x];
            pointers[x] = pointer_y;
        } else {
            println!("{}", a[pointers[x]][y]);
        }
    }
}