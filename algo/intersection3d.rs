use proconio::input;
// abc343e
// 三次元版のベン図
// 領域の相対的な位置が重要なので，一つの領域を固定する
// 全探索を行う
fn main() {
    // v1: 領域1つ, v2: 領域2つ, v3: C1 & C2 & C3
    input!{v1: i64, v2: i64, v3: i64}
    let (x1, y1, z1) = (0, 0, 0);
    for x2 in -7..=7 {
        for y2 in -7..=7 {
            for z2 in -7..=7 {
                for x3 in -7..=7 {
                    for y3 in -7..=7 {
                        for z3 in -7..=7 {
                            let coord: Vec<(i64, i64, i64)> = vec![(x1, y1, z1), (x2, y2, z2), (x3, y3, z3)];
                            // 領域のand演算
                            let c12 = intersection2(&coord, 0, 1);
                            let c23 = intersection2(&coord, 1, 2);
                            let c31 = intersection2(&coord, 2, 0);
                            let c123 = intersection3(&coord);

                            let cur_v3 = c123;
                            let cur_v2 = c12 + c23 + c31 - 3 * c123;
                            // 余事象の計算を行う
                            let cur_v1 = 3 * 7 * 7 * 7 - 2 * cur_v2 - 3 * cur_v3;
                            if cur_v1 == v1 && cur_v2 == v2 && cur_v3 == v3 {
                                println!("Yes");
                                println!("{x1} {y1} {z1} {x2} {y2} {z2} {x3} {y3} {z3}");
                                return;
                            }
                        }
                    }
                }
            }
        }
    }

    println!("No");
}

// 2つの立方体Ci,Cjの共通部分の体積V(Ci∩Cj)は、共通部分を表す式が
// max{ai,aj}≤x≤min{ai+7,aj+7}かつ
// max{bi,bj}≤y≤min{bi+7,bj+7}かつ
// max{ci,cj}≤z≤min{ci+7,cj+7}
// 上限 - 下限

fn intersection2(coord: &Vec<(i64, i64, i64)>, c1: usize, c2: usize) -> i64 {
    let mut res: i64 = 1;
    let (x1, y1, z1) = coord[c1];
    let (x2, y2, z2) = coord[c2];
    // xyz軸ごとに計算する
    res *= 0.max((x1+7).min(x2+7) - x1.max(x2));
    res *= 0.max((y1+7).min(y2+7) - y1.max(y2));
    res *= 0.max((z1+7).min(z2+7) - z1.max(z2));
    return res;
}

fn intersection3(coord: &Vec<(i64, i64, i64)>) -> i64 {
    let mut res: i64 = 1;
    let (x1, y1, z1) = coord[0];
    let (x2, y2, z2) = coord[1];
    let (x3, y3, z3) = coord[2];
    // xyz軸ごとに計算する
    res *= 0.max((x1+7).min(x2+7).min(x3+7) - x1.max(x2).max(x3));
    res *= 0.max((y1+7).min(y2+7).min(y3+7) - y1.max(y2).max(y3));
    res *= 0.max((z1+7).min(z2+7).min(z3+7) - z1.max(z2).max(z3));
    return res;
}