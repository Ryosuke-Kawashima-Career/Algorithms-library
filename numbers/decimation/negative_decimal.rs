use proconio::input;
// abc105C
// Q. -2進数に変換する
fn main() {
    input!{n: i64}
    let digits: Vec<i64> = change_negative_base(n, -2);
    for &digit in digits.iter().rev() {
        print!("{}", digit);
    }
    println!("");
}

// S=SkSk-1...S0とすると、N=S0×(-2)^0+S1×(-2)^1+...+Sk×(-2)^k
fn change_negative_base(mut num: i64, base: i64) -> Vec<i64> {
    let mut digits: Vec<i64> = Vec::new();
    // do-whileのように条件判定を後で行う
    for i in 1.. {
        let digit = if num % base.pow(i) == 0 {
            0
        } else {
            num -= base.pow(i-1);
            1
        };
        digits.push(digit);

        if num == 0 {
            break;
        }
    }
    return digits;
}

fn change_negative_decimation(mut num: i64, base: i64) -> Vec<i64> {
    let mut digits: Vec<i64> = Vec::new();
    if num == 0 {
        return vec![0];
    }
    while num != 0 {
        let digit = if num % (base.abs()) == 0 {
            0
        } else {
            1
        };
        digits.push(digit);

        if num < 0 {
            num = (num - base + 1) / base;
        } else {
            num = num / base;
        }
    }
    return digits;
}