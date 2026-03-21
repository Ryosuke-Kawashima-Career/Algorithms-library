use proconio::input;
// abc335C
// 蛇のゲームみたいに体のパーツが頭の動きについていくゲーム
// Dequeでパーツの座標を管理する
// なぜならパーツi(2≤i≤N)は移動前にパーツi-1があった座標に移動
fn main() {
    input!{n: usize, q: usize}
    let mut parts_pos = std::collections::VecDeque::new();
    for x in 1..=n {
        parts_pos.push_back((x as i64, 0));
    }
    let mut cur_x: i64 = 1;
    let mut cur_y: i64 = 0;
    let mut ans: Vec<(i64, i64)> = Vec::new();

    for _ in 0..q {
        input!{query: usize}
        if query == 1 {
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
            parts_pos.push_front((cur_x, cur_y));
            parts_pos.pop_back();
        } else {
            input!{p: usize}
            // 1indexed -> 0indexed
            ans.push((parts_pos[p-1].0, parts_pos[p-1].1));
        }
    }
    for &(x, y) in ans.iter() {
        println!("{} {}", x, y);
    }
}