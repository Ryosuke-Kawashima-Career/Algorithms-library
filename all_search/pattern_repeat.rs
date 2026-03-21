use proconio::input;
// abc346B
// 全探索する: O(w+b)
fn main() {
    input!{w: usize, b: usize}
    // patternをコピペして，むりやり計算する方法もある
    let pattern = "wbwbwwbwbwbw".to_string().chars().collect::<Vec<char>>();
    let n: usize = pattern.len();

    // 繰り返しのパターンの長さ分の全探索
    for start in 0..n {
        let (mut cnt_w, mut cnt_b) = (0, 0);
        // w+b個の部分列の全探索を行う
        for offset in 0..w+b {
            if pattern[(start+offset) % n] == 'w' {
                cnt_w += 1;
            } else {
                cnt_b += 1;
            }
        }
        if cnt_w == w && cnt_b == b {
            println!("Yes");
            return;
        }
    }
    println!("No");
}