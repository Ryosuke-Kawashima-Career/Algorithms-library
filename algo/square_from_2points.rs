use proconio::input;
use std::collections::HashSet;
// JOI第六回日本情報オリンピック本選C
// 頂点を二つ決めると正方形の形が二つに絞られる.(今回は反時計回りを見る)
// Edge: (x, y) -> (-y, x)で反時計回りの正方形
// Edge: (x, y) -> (y, -x)で時計回りの正方形
fn main() {
    input!{n: usize, xy: [(i64, i64); n]}
    let mut points = HashSet::new();
    for &point in xy.iter() {
        points.insert(point);
    }
    let mut max_area: i64 = 0;

    // 頂点を2つ選ぶ
    // i,jの中点oを求める -> ベクトルio = (dx, dy)
    // dx = (x2 - x1) / 2, dy = (y2 - y1) / 2
    // 中点o + (-dy, dx) or (dy, -dx)
    for i in 0..n {
        for j in (i+1)..n {
            // x = xj - yj + yi, y = yj + xj - xi
            let vertex1 = (xy[j].0 - xy[j].1 + xy[i].1, xy[j].1 + xy[j].0 - xy[i].0);
            // x = xi - yj + yi, y = yi + xj - xi
            let vertex2 = (xy[i].0 + xy[i].1 - xy[j].1, xy[i].1 + xy[j].0 - xy[i].0);

            if points.contains(&vertex1) && points.contains(&vertex2) {
                let area: i64 = (xy[i].0 - xy[j].0) * (xy[i].0 - xy[j].0) + (xy[i].1 - xy[j].1) * (xy[i].1 - xy[j].1);
                max_area = max_area.max(area);
            }
        }
    }

    println!("{}", max_area);
}