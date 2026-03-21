use proconio::input;
// abc338E
// 曲線同士の交点が存在しないことは
// Ai<Aj<Bi<Bj を満たすi,j(i!=j)が存在しないことと同値
// 積で表現する方法もある(product1 / product1.abs()) * (product2 / product2.abs()) < 0はバグる
/*
let product1 = (lines[(i+1) % n].0 - lines[i].0) * (lines[(i+1) % n].1 - lines[i].1);
let product2 = (lines[(i+1) % n].1 - lines[i].0) * (lines[(i+1) % n].0 - lines[i].1);
if (product1 > 0) != (product2 > 0) {
    is_crossed = true;
}
*/

// 円を直線に変換する
// stackで交点の存在を計算する: ()をstackで管理するのと同じ
fn main() {
    input!{n: usize, ab: [(usize, usize); n]}
    // 0: start, 1: end: (start or end, index)
    let mut points: Vec<(usize, usize)> = vec![(0, 0); 2*n];
    for i in 0..n {
        if ab[i].0 < ab[i].1 {
            points[ab[i].0 - 1] = (0, i);
            points[ab[i].1 - 1] = (1, i);
        } else {
            points[ab[i].1 - 1] = (0, i);
            points[ab[i].0 - 1] = (1, i);
        }
    }

    let mut is_crossed: bool = false;
    let mut stack = Vec::new();
    for i in 0..2*n {
        let (state, id) = points[i];
        if state == 0 {
            stack.push(id);
        } else {
            if let Some(&top_id) = stack.last() {
                if top_id != id {
                    is_crossed = true;
                }
                stack.pop();
            }
        }
    }
    if is_crossed {
        println!("Yes");
    } else {
        println!("No");
    }
}