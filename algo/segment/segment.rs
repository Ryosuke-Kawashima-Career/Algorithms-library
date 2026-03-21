use proconio::input;
// abc349D
// セグメント木の区間の取り方を応用する
// Span(l, r) = [l, r) = Span(2^i*j, 2^i*(j+1))
fn main() {
    input!{l: usize, r: usize}
    let answer: Vec<(usize, usize)> = segments(l, r);
    print_answer(&answer);
}

fn segments(mut l: usize, r: usize) -> Vec<(usize, usize)> {
    let mut res: Vec<(usize, usize)> = Vec::new();
    
    while l < r {
        let mut power: u32 = 0;
        // 1. 2^powerがlを超えないようにする
        // 2. lがrを超えないようにする
        while l % 2usize.pow(power + 1) == 0
        && l + 2usize.pow(power+1) <= r {
            power += 1;
        }
        res.push((l, l + 2usize.pow(power)));
        l += 2usize.pow(power);
    }

    return res;
}

fn print_answer(answer: &Vec<(usize, usize)>) {
    let k: usize = answer.len();
    println!("{}", k);
    for &(l, r) in answer.iter() {
        println!("{} {}", l, r);
    }
}