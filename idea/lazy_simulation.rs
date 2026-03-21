use proconio::input;
use std::{collections::BTreeMap, mem::swap};
// abc417D
// Q. if the tension is no more than P, it increases by A, and otherwise decreases by B.
// A. paraphrase the query!
// 1. 現在のテンションがP以下のとき、テンションをA+Bだけ増加させる。
// 2. さらに、テンションをBだけ減少させる。(Bを後で処理する)
fn main() {
    input!{n: usize, pab: [(usize, usize, usize); n], q: usize, x: [usize; q]}
    // key: tension, value: corresponding Queries!!
    let mut map = BTreeMap::<usize, Vec<usize>>::new();
    for query in 0..q {
        let tension: usize = x[query];
        map.entry(tension).or_default().push(query);
    }
    let mut accumulated_decrease: usize = 0;
    // [(tension, [Queries])]
    let mut tensions_and_queries: Vec<(usize, Vec<usize>)> = Vec::new();

    for (present_value, a_increase, b_decrease) in pab.iter() {
        // tension - accumulated_decrease <= p is transformed!!
        while let Some((&tension, _)) = map.range(..=present_value + accumulated_decrease).next() {
            let new_queries = map.remove(&tension).unwrap();
            tensions_and_queries.push((tension.saturating_sub(accumulated_decrease) + accumulated_decrease + a_increase + b_decrease, new_queries));
        }

        while let Some((tension, mut new_queries)) = tensions_and_queries.pop() {
            map.entry(tension)
                .and_modify(|queries| {
                    // Append the shorter one
                    if queries.len() < new_queries.len() {
                        swap(queries, &mut new_queries);
                    }
                    queries.extend(&new_queries);
                })
                .or_insert(new_queries);
        }
        accumulated_decrease += b_decrease;
    }
    let mut answer: Vec<usize> = vec![0; q];
    for (tension, queries) in map {
        for query in queries {
            // Lazy Evaluation
            answer[query] = tension.saturating_sub(accumulated_decrease);
        }
    }
    for ans in answer {
        println!("{}", ans);
    }
}