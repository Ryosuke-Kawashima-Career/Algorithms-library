use proconio::input;
// abc093C: 不変量を見つける(parity)
// Q. A,B,C を以下の操作ですべて等しくする
// ① 2つを選んで、その両方を1増やす ② 1つを選んで、その整数を2増やす
// A. 3つの整数の和の偶奇は操作によって不変
// 操作で数が減ることがないので, 最終的な値はM(max(A,B,C))かM+1となる.
// つまり答えの下限値は 最終値 * 3 - (A+B+C) を2で割った値
fn main() {
    input!{a: i64, b: i64, c: i64}
    let parity = (a + b + c) % 2;
    let m = a.max(b).max(c);
    // 下限値を見積もる
    let diff = if (3 * m) % 2 == parity {
        3 * m - (a + b + c)
    } else {
        3 * (m + 1) - (a + b + c)
    };
    println!("{}", diff / 2);
}