// 鉄則C08
fn main() {
    let mut all_nums: Vec<Vec<char>> = Vec::new();
    enum_all(4, vec![], &mut all_nums);
}

// 数字の個数, 答えの数, 答えの配列
fn enum_all(remain: usize, mut num: Vec<char>, all: &mut Vec<Vec<char>>) {
    if remain == 0 {
        all.push(num);
        return;
    }

    for j in 0..10 {
        let digit: char = char::from_digit(j, 10).unwrap();
        num.push(digit);
        enum_all(remain-1, num.clone(), all);
        num.pop();
    }
}