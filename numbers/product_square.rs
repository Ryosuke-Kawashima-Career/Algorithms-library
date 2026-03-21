use proconio::input;
// abc342d
// 式変形をして問題を言い換える
// 整数xについて、xを割り切る最大の平方数をdxとする
// Ax*Ayが平方数　<=>  Ax/dx * Ay/dyが平方数
// <=> Ax/dx = Ay/dy
fn main() {
    input!{n: usize, a: [usize; n]}
    let mut da: Vec<usize> = Vec::new();
    for i in 0..n {
        da.push(get_max_square(a[i]));
    }
    let mut a_da = std::collections::HashMap::new();
    for i in 0..n {
        // 0は例外
        if da[i] == 0 {
            *a_da.entry(0).or_insert(0) += 1;
        } else {
            *a_da.entry(a[i]/da[i]).or_insert(0) += 1;
        }
    }

    let mut ans: usize = 0;
    for (&key, &val) in a_da.iter() {
        if key == 0 {
            ans += val * (n - val);
        }
        if val > 0 {
            ans += val * (val - 1) / 2;
        }
    }
    println!("{}", ans);
}

fn get_max_square(num: usize) -> usize {
    if num == 0 {
        return 0;
    }
    let mut max_square = 1;
    let mut d: usize = 1;
    while d * d <= num {
        if num % (d * d) == 0 {
            max_square = d * d;
        }
        d += 1;
    }
    return max_square;
}