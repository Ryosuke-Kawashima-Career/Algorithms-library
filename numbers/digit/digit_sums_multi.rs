use proconio::input;
// 鉄則B37
// 小さな数を手で実験して、場合分け
fn main() {
    input!{n: usize}
    let mut pow10: Vec<usize> = vec![1; 17];
    for i in 1..=16 {
        pow10[i] = pow10[i-1] * 10;
    }
    // 数字jのi桁目の個数 = count[i][j]
    let mut count: Vec<Vec<usize>> = vec![vec![0; 10]; 16];

    for i in 0..16 {
        // nの下からi桁目(0index)の数を計算する。
        let digit: usize = (n / pow10[i]) % 10;
        // i桁目の数がjの出現回数を計算する。
        for j in 0..10 {
            if j < digit {
                count[i][j] = n / pow10[i+1] * pow10[i] + pow10[i];
            } else if j == digit {
                count[i][j] = n / pow10[i+1] * pow10[i] + n % pow10[i] + 1;
            } else {
                count[i][j] = n / pow10[i+1] * pow10[i];
            }
        }
    }

    let mut digit_sums: usize = 0;
    // 各桁iの数字jの寄与を計算する。
    for i in 0..16 {
        for j in 0..10 {
            digit_sums += count[i][j] * j;
        }
    }

    println!("{}", digit_sums);
}