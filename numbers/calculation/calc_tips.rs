use std::f64::consts::PI;
const RAD_TO_DEG: f64 = std::f64::consts::FRAC_1_PI * 180.0;
const DEG_TO_RAD: f64 = std::f64::consts::PI / 180.0;

const MOD: i64 = 1_000_000_007;
const INF: f64 = 1e9 + 7.0;
const DAY: i64 = 60 * 60 * 24;

fn main() {
    // [/]: a / b では正負によらず0に近いほうに丸めこまれる
    // [%]: (a / b * b) + (a % b) == a
    assert_eq!(5 / 4 == 1 && 5 % 4 == 1);
    assert_eq!(5 / -4 == -1 && 5 % -4 == 1);
    assert_eq!(-5 / 4 == -1 && -5 % 4 == -1);
    assert_eq!(-5 / -4 == 1 && 5 % 4 == -1);
    // xorは同じ数の演算で0になる 
    // Ex. a ^ a == 0, Ex. a ^ a ^ b == b
    // n ^ n+1 == 1
    
    // max of multi
    let max_num: usize = num1.max(num2).max(num3);
    // a / (b*c)
    let divided = a / b / c;
    // %と*の優先度は同じらしい
    let mod_calc = a % b * c % d;
    // int
    // overflow
    let inf: usize = 0 - 1;
    // f64
    let rad: f64 = PI;
    let deg: f64 = PI * RAD_TO_DEG;
    // T: 周期
    let rad_sin = (2.0 * PI * t / T).sin();

    // calc sqrt(桁落ちに気を付ける)
    let sqrt_n: usize = (n as f64).sqrt() as usize + 1;

    // ceil and floor
    // ceil = (num + n - 1) / n
    // a から a / 2 と a - a / 2 を得る操作は a から ⌊a/2⌋ と ⌈a/2⌉を得る操作
    let floor = num / 2;
    let ceil = (num + 1) / 2;

    // int is more precise
    // loga > blogc <=> a > c ^ b

    // MOD
    let modint: i64 = (val1 - val2 + MOD) % MOD;
    // valをMOD上で1減らす(+ (MOD - 1))
    let dec: i64 = (val + MOD - 1) % MOD;

    // time calc
    // 秒に変換して計算する
    let mut time_second: i64 = second + 60 * minute + 60 * 60 * hour;
    time_second += diff_second;
    time_second %= DAY;
    if time_second < 0 {
        time_second += DAY;
    }

    let hour = time_second / 3600;
    time_second %= 3600;
    let minute = time_second / 60;
    time_second %= 60;
    let second = time_second;

    // Ex. i=2: 10..=99(=100-1)
    let num_l: usize = l.max(power10[i-1]);
    let num_r: usize = r.min(power10[i]-1);
}