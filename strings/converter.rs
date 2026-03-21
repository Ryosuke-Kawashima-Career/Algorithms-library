fn char_to_num(c: char) -> usize {
    let num: usize = c as usize - 48;
    return num;
}

fn to_lowercase(c: char) -> char {
    if c as u8 - ('a' as u8) < 26 {
        return c;
    }
    return (c as u8 - 'A' as u8 + 'a' as u8) as char;
}

fn to_uppercase(c: char) -> char {
    if c as u8 - ('a' as u8) >= 26 {
        return c;
    }
    return (c as u8 - 'a' as u8 + 'A' as u8) as char;
}

fn main() {
    let to_num = |c: char| -> usize {
        match c {
            'J' => 0,
            'O' => 1,
            'I' => 2,
            _ => {
                println!("Input Error!");
                3
            },
        }
    };
    let s: char = '1';
    let s_string = format!("{}", s);
    let s_num = char_to_num(s);
    println!("{}", s_num);
    // Chars -> String
    let string = chars.iter().collect::<String>();
    let string_rev = chars.iter().rev().collect::<String>();
}