// 「最小値を最大化してください」という問題では二分探索が有効
fn main() {
    while right - left > 1 {
        let mid: usize = (left + right) / 2;
        if is_ok(mid) {
            right = mid;
        } else {
            left = mid;
        }
    }
}