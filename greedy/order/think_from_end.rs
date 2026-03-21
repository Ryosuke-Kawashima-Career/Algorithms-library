use proconio::input;
// abc137d
// 貪欲法: sortと相性がいい -> Heapとも相性がいい
fn main() {
    input!{n: usize, m: usize, ab: [(usize, i64); n]}
    let mut rewards: i64 = 0;
    // 締め切りから(後ろから)決定する。
    // 候補の選択肢が締め切りから遠ざかると増えてくる。
    let mut from_dues: Vec<Vec<i64>> = vec![vec![]; m+1];
    for &(time, reward) in ab.iter() {
        if time <= m {
            from_dues[time].push(reward);
        }
    }

    // heap(仕事の報酬)
    let mut heap = std::collections::BinaryHeap::new();
    // 時をさかのぼる
    for day in (0..=m).rev() {
        for &task in from_dues[m-day].iter() {
            heap.push(task);
        }

        if let Some(reward) = heap.pop() {
            rewards += reward;
        }
    }
    
    println!("{}", rewards);
}