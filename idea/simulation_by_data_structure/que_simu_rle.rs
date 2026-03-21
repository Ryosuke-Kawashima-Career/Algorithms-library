use proconio::input;
use std::collections::VecDeque;
// abc446C
// Q. a[]: input number of eggs b[]: output number of eggs
// A. Simulation by using a queue(The label is Run Length Encoding)
// keypoints: A_i <= 10
fn main() {
    input! {t: usize}
    for _case in 0..t {
        input! {n: usize, d: usize, a: [usize; n], b: [usize; n]}
        // VecDeque<(added_day, num_eggs)>
        let mut que: VecDeque<(usize, usize)> = VecDeque::new();
        for day in 0..n {
            // in the morning, add eggs
            que.push_back((day, a[day]));
            // in the evening, remove eggs
            let mut eggs_to_remove = b[day];
            while eggs_to_remove > 0 {
                let (added_day, num_eggs) = que.pop_front().unwrap();
                if num_eggs <= eggs_to_remove {
                    eggs_to_remove -= num_eggs;
                } else {
                    que.push_front((added_day, num_eggs - eggs_to_remove));
                    eggs_to_remove = 0;
                }
            }
            // remove eggs that are older than d days
            while let Some((added_day, _)) = que.front() {
                if day - added_day >= d {
                    que.pop_front();
                } else {
                    break;
                }
            }
        }
        let mut num_remaining_eggs = 0;
        for (_, num_eggs) in que {
            num_remaining_eggs += num_eggs;
        }
        println!("{}", num_remaining_eggs);
    }
}
