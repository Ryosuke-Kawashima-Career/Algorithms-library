use proconio::input;
// JOI2008予選4(星座探し)
// pattern match: hashsetなどのデータ構造で高速化する
fn main() {
    input!{m: usize, target: [(i64, i64); m], n: usize, stars: [(i64, i64); n]}
    let mut coordinate = std::collections::HashSet::new();
    for &star in stars.iter() {
        coordinate.insert(star);
    }

    // 星と星を対応させる
    for i in 0..m {
        for j in 0..n {
            // target[i]の点をstars[j]に平行移動させる
            let dx: i64 = stars[j].0 - target[i].0;
            let dy: i64 = stars[j].1 - target[i].1;

            let mut all_match: bool = true;
            for k in 0..m {
                if !coordinate.contains(&(target[k].0 + dx, target[k].1 + dy)) {
                    all_match = false;
                }
            }

            if all_match {
                println!("{} {}", dx, dy);
                return;
            }
        }
    }
}