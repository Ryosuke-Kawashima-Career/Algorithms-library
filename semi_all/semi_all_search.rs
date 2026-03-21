use proconio::input;
use superslice::Ext;
// 鉄則A14
// 半分全列挙
fn main() {
    input!{n: usize, k: i64, a: [i64; n], b: [i64; n], c: [i64; n], d: [i64; n]}
    // 配列a, bを組み合わせる。
    let mut ab: Vec<i64> = vec![0; n*n];
    let mut cd: Vec<i64> = vec![0; n*n];
    for y in 0..n {
        for x in 0..n {
            // n*y+xでジャグ配列 vector[y][x]と同じ。
            ab[n*y+x] = a[y] + b[x];
            cd[n*y+x] = c[y] + d[x];
        }
    }

    cd.sort();
    let mut is_ok: bool = false;
    for i in 0..(n*n) {
        // ab[i]以上となるindex
        let cd_value: i64 = k - ab[i];
        let idx: usize = cd.lower_bound(&cd_value);
        if idx < n*n && ab[i] + cd[idx] == k {
            is_ok = true;
            break;
        }
    }

    if is_ok {
        println!("Yes");
    } else {
        println!("No");
    }
}