use proconio::input;
// abc336c
// 0, 2, 4, 6, 8のみ -> 0, 1, 2, 3, 4
// 5進法に変換して計算する
fn main() {
    input!{n: usize}
    let deci5: usize = change_decimation(n-1, 10, 5);
    let digits: Vec<char> = deci5.to_string().chars().collect();
    for &digit in digits.iter() {
        let num: usize = digit as usize - '0' as usize;
        print!("{}", num * 2);
    }
    println!("");
}

fn change_decimation(num: usize, before: usize, after: usize) -> usize {
    // before -> decimal
    let mut decimal: usize = usize::from_str_radix(&num.to_string(), before as u32).unwrap();
    if decimal == 0 {
        return 0;
    }

    // decimal -> after
    let mut digits: Vec<char> = Vec::new();
    while decimal > 0 {
        let digit: usize = decimal % after;
        digits.push(char::from_digit(digit as u32, 10).unwrap());
        decimal /= after;
    }

    let res_string = digits.iter().rev().collect::<String>();
    let res: usize = res_string.parse().unwrap();
    return res;
}