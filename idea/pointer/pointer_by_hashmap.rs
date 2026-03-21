use proconio::input;
use std::collections::HashMap;
const INF: usize = 1 << 60;
// abc344E
// Linked List(双方向リスト): 要素の挿入と削除を高速に行う
// pointerをHashMapで管理する
// 0 -> node -> INF
fn main() {
    input!{n: usize, a: [usize; n], q: usize}
    // 0 -> a[0] -> a[n-1] -> INF
    // あるノードの前のノードを格納する
    let mut before_nodes = HashMap::new();
    before_nodes.insert(a[0], 0);
    before_nodes.insert(INF, a[n-1]);
    // あるノードの後のノードを格納する
    let mut after_nodes = HashMap::new();
    after_nodes.insert(0, a[0]);
    after_nodes.insert(a[n-1], INF);
    for i in 0..n-1 {
        before_nodes.insert(a[i+1], a[i]);
        after_nodes.insert(a[i], a[i+1]);
    }

    for _ in 0..q {
        input!{query: usize}
        if query == 1 {
            // insert: x -> y -> next_x
            input!{x: usize, y: usize}
            let next_x = after_nodes[&x];
            before_nodes.insert(y, x);
            after_nodes.insert(y, next_x);
            if let Some(node) = after_nodes.get_mut(&x) {
                *node = y;
            }
            if let Some(node) = before_nodes.get_mut(&next_x) {
                *node = y;
            }
        } else {
            // erase x: pre_x -> x -> next_x
            input!{x: usize}
            let next_x = after_nodes[&x];
            let pre_x = before_nodes[&x];
            before_nodes.remove(&x);
            after_nodes.remove(&x);
            if let Some(node) = before_nodes.get_mut(&next_x) {
                *node = pre_x;
            }
            if let Some(node) = after_nodes.get_mut(&pre_x) {
                *node = next_x;
            }
        }
    }

    // repeat until reach nil!
    let mut cur_x: usize = 0;
    loop {
        cur_x = after_nodes[&cur_x];
        if cur_x == INF {
            break;
        }
        print!("{} ", cur_x);
    }
}