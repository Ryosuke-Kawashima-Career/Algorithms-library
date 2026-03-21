use proconio::input;
use std::cmp::Reverse;
// abc335D
// 貪欲法: priority-que(Heap)で管理する
// 開始できる仕事をBTreeMapで管理する
fn main() {
    input!{n: usize, td: [(usize, usize); n]}
    // (start, end)
    let mut times: Vec<Task> = Vec::new();
    // key: start time, val: [tasks]
    let mut start_tasks = std::collections::BTreeMap::new();
    for i in 0..n {
        // start + span
        times.push(Task::new(td[i].0, td[i].0 + td[i].1));
    }
    // 終了時間が速い順に並べる
    times.sort();
    for task in 0..n {
        start_tasks.entry(times[task].start).or_insert(vec![]).push(task);
    }
    // deadlines
    let mut deadlines = std::collections::BinaryHeap::new();
    let mut ans: usize = 0;
    let mut cur_time: usize = 0;
    // loop: 時間を進める
    loop {
        // 開始できる仕事をHeapに乗せる
        if let Some((&start_time,_)) = start_tasks.iter().next() {
            if start_time <= cur_time {
                let tasks = start_tasks.pop_first().unwrap().1;
                for &task in tasks.iter() {
                    deadlines.push(Reverse(times[task].end));
                }
            }
        }

        // 締め切りを過ぎた仕事をあきらめる
        while let Some(&Reverse(end_time)) = deadlines.peek() {
            if end_time < cur_time {
                deadlines.pop();
            } else {
                // it is needed to stop this while loop! 
                break;
            }
        }

        // 制限時間が迫っている仕事
        if let Some(_) = deadlines.pop() {
            ans += 1;
        } else {
            // if deadlines is empty
            if let Some((&next_time, _)) = start_tasks.iter().next() {
                cur_time = next_time;
                continue;
            } else {
                break;
            }
        }

        cur_time += 1;
    }
    println!("{}", ans);
}

#[derive(Debug, Copy, Clone, Eq)]
struct Task {
    start: usize, end: usize
}

impl Task {
    fn new(start_time: usize, end_time: usize) -> Self {
        Self{start: start_time, end: end_time}
    }
}

impl PartialOrd for Task {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self.end == other.end {
            (other.start).partial_cmp(&self.start)
        } else {
            (other.end).partial_cmp(&self.end)
        }
    }
}
impl Ord for Task {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.end == other.end {
            (other.start).cmp(&self.start)
        } else {
            (other.end).cmp(&self.end)
        }
    }
}

impl PartialEq for Task {
    fn eq(&self, other: &Self) -> bool {
        self.start == other.start && self.end == other.end
    }
}