use proconio::input;
// abc311E
// 二分探索で二次元累積和の計算を高速化する(穴の数が正方形の辺の長さと共に増加(単調性))
// h*wのマス目に(a, b)に穴をあける.穴のない正方形の区間の組み合わせを求める
fn main() {
    input!{h: usize, w: usize, n: usize, ab: [(usize, usize); n]}
    let mut prefix: Vec<Vec<usize>> = vec![vec![0; w+1]; h+1];
    init_prefix(&mut prefix, &ab);
    let mut ans = 0;

    for i in 1..=h {
        for j in 1..=w {
            ans += binary_prefix(i, j, &prefix);
        }
    }
    println!("{}", ans);
}
// (y, x): 左上のマス span: 正方形の辺の長さ
fn binary_prefix(y: usize, x: usize, prefix: &Vec<Vec<usize>>) -> usize {
    let h: usize = prefix.len()-1;
    let w: usize = prefix[0].len()-1;
    let mut span_l: usize = 0;
    let mut span_r: usize = (h-y+1).min(w-x+1)+1;
    
    while span_r - span_l > 1 {
        let span_mid: usize = (span_l + span_r) / 2;
        let num_holes = prefix[y-1][x-1] + prefix[y+span_mid-1][x+span_mid-1] 
        - prefix[y-1][x+span_mid-1] - prefix[y+span_mid-1][x-1];
        if num_holes == 0 {
            span_l = span_mid;
        } else {
            span_r = span_mid;
        }
    }
    
    return span_l;
}

fn init_prefix(prefix: &mut Vec<Vec<usize>>, ab: &Vec<(usize, usize)>) {
    let h: usize = prefix.len()-1;
    let w: usize = prefix[0].len()-1;
    for &(a, b) in ab.iter() {
        prefix[a][b] += 1;
    }
    for i in 1..=h {
        for j in 1..=w {
            prefix[i][j] += prefix[i][j-1];
        }
    }
    for i in 1..=h {
        for j in 1..=w {
            prefix[i][j] += prefix[i-1][j];
        }
    }
}