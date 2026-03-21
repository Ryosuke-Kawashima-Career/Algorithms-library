use proconio::{input, marker::Chars};
const MOD: usize = 1_000_000_007;
// 第一回日本最強プログラマー学生選手権-予選- C: 区間を反転する
// 1. 同じ操作を2回やると元に戻る
// 2. 操作の順番を入れ替えても同じ
// 3. l1-l2-r1-r2: [l1, r1]と[l2, r2] の反転は、[l1, r2]と[l2,r1] の反転と同じ（区間の左端はどのように反転させても左端、右端はどのように反転させても右端）
// 4. 反転区間の端点に注目するとgreedyに決まることがある
// Q. 白黒の配列の区間をN回反転(すべてのマスを1回ずつ選ぶ)させ，すべてを白にする方法の場合の数
// A. 隣り合う2つの頂点に注目し、反転区間のどちらの端点になるか調べる
// A. 操作の順序は関係ない: 2つの操作(l1, r1), (l2, r2) = (l1, r2), (l2, r1)
// A. 各マスが左側として選ばれたのか右側として選ばれたのかのみによって結果が変わる
fn main() {
    input!{n: usize, s: Chars}
    let mut fact: Vec<usize> = vec![1; n+1];
    for i in 1..=n {
        fact[i] = (fact[i-1] * i) % MOD;
    }
    let colors = s.iter().map(|&c| if c == 'B' {
        1
    } else {
        0
    }).collect::<Vec<usize>>();

    // 初期値は白
    let mut color_cur: usize = 0;
    let mut num_left: usize = 0;
    let mut ans: usize = 1;
    for i in 0..2*n {
        let is_different = (colors[i] + color_cur) % 2;
        if is_different == 1 {
            num_left += 1;
        } else {
            if num_left > 0 {
                ans = (ans * num_left) % MOD;
                num_left -= 1;
            } else {
                println!("0");
                return;
            }
        }
        // 色を反転する
        color_cur = (color_cur + 1) % 2;
    }

    if num_left == 0 {
        // 操作は必ずN回行うので、操作の順番を入れ替えた場合も考慮
        ans = (ans * fact[n]) % MOD;
    } else {
        // leftとrightの数が釣り合わないとき
        ans = 0;
    }
    println!("{}", ans);
}