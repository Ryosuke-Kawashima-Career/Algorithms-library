use proconio::input;
// abc349D
// セグメントツリーの再帰的な実装のようにトップダウンに再帰する
// Span(l, r) = [l, r) = Span(2^i*j, 2^i*(j+1))
// 良い数列[L,R)に対し、それと入力で与えられた数列[l,r)の共通部分の最適な分割をf(l,r,L,R)
fn main() {
    input!{l: usize, r: usize}
    let answer: Vec<(usize, usize)> = seg_rec(l, r, 0, 1 << 60);
    print_answer(&answer);
}

// [L R)を半分ずつ二分探索のように狭める感じ
fn seg_rec(l: usize, r: usize, L: usize, R: usize) -> Vec<(usize, usize)> {
    if l <= L && R <= r {
        return vec![(L, R)];
    }
    let M: usize = (L + R) / 2;
    // L < l < r < M < R
    if r <= M {
        return seg_rec(l, r, L, M);
    }
    // L < M < l < r < R
    if M <= l {
        return seg_rec(l, r, M, R);
    }

    let mut left = seg_rec(l, r, L, M);
    let mut right = seg_rec(l, r, M, R);
    left.append(&mut right);
    return left;
}

fn print_answer(answer: &Vec<(usize, usize)>) {
    let k: usize = answer.len();
    println!("{}", k);
    for &(l, r) in answer.iter() {
        println!("{} {}", l, r);
    }
}