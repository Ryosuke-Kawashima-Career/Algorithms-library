fn change_decimation(num: usize, before: usize, after: usize) -> usize {
    // before -> decimal
    let mut decimal: usize = usize::from_str_radix(&num.to_string(), before as u32).unwrap_or(0);
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

// calc by string
fn radix_conv(num: String, before: usize, after: usize) -> String {
    // before -> decimal
    let mut num_decimal: usize = usize::from_str_radix(&num, before as u32).unwrap_or(0);
    if num_decimal == 0 {
        return "0".to_string();
    }

    // decimal -> after
    let mut digits = String::new();
    while num_decimal > 0 {
        let digit: usize = num_decimal % after;
        digits.push_str(&format!("{}", digit));
        num_decimal /= after;
    }
    let num_after_string = digits.chars().rev().collect::<String>();
    return num_after_string;
}