use proconio::input;
// abc349D
// セグメント木の区間の取り方を応用する
// Span(l, r) = [l, r) = Span(2^i*j, 2^i*(j+1))
// 区間は山型（右上がり→右下がり）になるという性質
// Span(l1,r1)=(2^i1*j1,2^i1(j1+1)) ,…, Span(lM,rM) = (2^iM*jM,2^iM(jM+1))としたとき
// ある整数kが存在してi1<⋯<ik>…>iMになっています
fn main() {
    input!{l: usize, r: usize}
    let answer: Vec<(usize, usize)> = segments(l, r);
    print_answer(&answer);
}

fn segments(mut l: usize, r: usize) -> Vec<(usize, usize)> {
    let mut res: Vec<(usize, usize)> = Vec::new();
    // 右上がり
    for i in 0..=60 {
        if l % 2usize.pow(i+1) == 2usize.pow(i) && l + 2usize.pow(i) <= r {
            res.push((l, l + 2usize.pow(i)));
            l += 2usize.pow(i);
        }
    }
    // 右下がり
    for i in (0..=60).rev() {
        if l + 2usize.pow(i) <= r {
            res.push((l, l + 2usize.pow(i)));
            l += 2usize.pow(i);
        }
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