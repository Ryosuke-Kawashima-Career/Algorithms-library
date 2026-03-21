use proconio::input;
// abc335C
// 蛇のゲームみたいに体のパーツが頭の動きについていくゲーム
// 頭の動きを履歴に記録する
// パーツi(2≤i≤N)は移動前にパーツi-1があった座標に移動
fn main() {
    input!{n: usize, q: usize}
    let mut head_pos: Vec<(i64, i64)> = Vec::new();
    for x in (1..=n).rev() {
        head_pos.push((x as i64, 0));
    }
    // index: n-1 = (time = 0)
    let offset: usize = n-1;
    let mut time: usize = 0;
    let mut cur_x: i64 = 1;
    let mut cur_y: i64 = 0;
    let mut ans: Vec<(i64, i64)> = Vec::new();

    for _ in 0..q {
        input!{query: usize}
        if query == 1 {
            time += 1;
            input!{c: char}
            let (dx, dy) = match c {
                'R' => (1, 0),
                'L' => (-1, 0),
                'U' => (0, 1),
                'D' => (0, -1),
                _ => {
                    println!("Input Error!");
                    (1, 1)
                },
            };
            cur_x += dx;
            cur_y += dy;
            head_pos.push((cur_x, cur_y));
        } else {
            input!{p: usize}
            // パーツpの時間に対応する頭の時間を導出する!!!
            let index: usize = offset + time - (p - 1);
            ans.push((head_pos[index].0, head_pos[index].1));
        }
    }
    for &(x, y) in ans.iter() {
        println!("{} {}", x, y);
    }
}