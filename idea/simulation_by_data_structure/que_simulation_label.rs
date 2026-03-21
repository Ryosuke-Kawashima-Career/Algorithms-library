use proconio::input;
use std::collections::VecDeque;
// abc446C
// Q. a[]: input number of eggs b[]: output number of eggs
// A. Simulation by using a queue (The label is the added day)
// keypoints: A_i <= 10
fn main() {
    input! {t: usize}
    for _case in 0..t {
        input! {n: usize, d: usize, a: [usize; n], b: [usize; n]}
        // que has the days when the eggs were added
        let mut que: VecDeque<usize> = VecDeque::new();
        for day in 0..n {
            for _ in 0..a[day] {
                que.push_back(day);
            }
            for _ in 0..b[day] {
                que.pop_front();
            }
            while let Some(added_day) = que.front() {
                if day - added_day >= d {
                    que.pop_front();
                } else {
                    break;
                }
            }
        }
        let num_remaining_eggs = que.len();
        println!("{}", num_remaining_eggs);
    }
}
