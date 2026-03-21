// priority que in C++
use std::cmp::Reverse;

fn main() {
    // pop max
    let mut heap = std::collections::BinaryHeap::new();
    heap.push(1);
    if let Some(value) = heap.pop() {
        // procedures
    }
    while let Some(Reverse(value)) = priority_que.pop() {
    }

    // pop min
    heap.push(Reverse(1));
    if let Some(Reverse(value)) = heap.pop() {
        
    }

    // example
    input!{q: usize}
    // default key: max
    // min: Reverse or -value
    let mut priority_que = std::collections::BinaryHeap::new();
    for _ in 0..q {
        input!{query: usize}
        if query == 1 {
            input!{x: usize}
            priority_que.push(Reverse(x));
        } else if query == 2 {
            // Reverseを.0で解除
            let min: usize = priority_que.peek().unwrap().0;
            
            if let Some(Reverse(x)) = que.peek() {
                println!("{}", x);
            }
            println!("{}", min);
        } else {
            priority_que.pop();
        }
    }

    // 条件を満たす間pop
    while let Some(val) = bh.peek() {
        if *val <= limit {
            let pop_val = bh.pop().unwrap();
        } else {
            break;
        }
    }
}