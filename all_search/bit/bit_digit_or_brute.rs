use proconio::input;
const MOD: usize = 1_000_000_007;
// 典型86
// bitごとに独立に演算を考える(and, or, xor)
fn main() {
    input!{n: usize, q: usize}
    let mut queries: Vec<(usize, usize, usize, usize)> = Vec::new();
    for _ in 0..q {
        input!{x: usize, y: usize, z: usize, w: usize}
        // a[x] | a[y] | a[z] == w ?
        queries.push((x-1, y-1, z-1, w));
    }
    let mut ans: usize = 1;

    // i: bitの桁
    for i in 0..60 {
        let valid_cases: usize = bit_brute(i, n, &queries);
        ans *= valid_cases;
        ans %= MOD;
    }

    println!("{}", ans % MOD);
}

// n: 数列の長さ
fn bit_brute(digit: usize, n: usize, queries: &Vec<(usize, usize, usize, usize)>) -> usize {
    let mut ways: usize = 0;
    // 数列のどの数のi桁目のbitが立っているか ex. 010(index=1の部分の数だけbitが立っている)
    for bit in 0..(1 << n) {
        let mut all_correct: bool = true;
        // x,y,z: 0..n -> 桁数に注目する
        // w: 0..60
        for &(x, y, z, w) in queries.iter() {
            let correct_bit: usize = (w >> digit) & 1;
            let x_bit: usize = (bit >> x) & 1;
            let y_bit: usize = (bit >> y) & 1;
            let z_bit: usize = (bit >> z) & 1;
            // bitごとにorの演算をする
            if correct_bit != (x_bit | y_bit | z_bit) {
                all_correct = false;
            }
        }

        if all_correct {
            ways += 1;
        }
    }

    return ways;
}