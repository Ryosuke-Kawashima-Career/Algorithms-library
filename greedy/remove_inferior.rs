use proconio::input;
// 鉄則A60
// 下位互換を消しても悪化しない -> 貪欲法
fn main() {
    input!{n: usize, a: [i64; n]}
    let mut stack: Vec<(usize, i64)> = Vec::new();

    // 株価の低い完全下位互換の日を駆逐する。
    for i in 0..n {
        while let Some((day, stock)) = stack.pop() {
            if stock > a[i] {
                print!("{} ", day);
                stack.push((day, stock));
                stack.push((i+1, a[i]));
                break;
            }
        }
        if stack.is_empty() {
            stack.push((i+1, a[i]));
            print!("-1 ");
        }
    }
}