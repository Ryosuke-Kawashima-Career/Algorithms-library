use proconio::input;
// 鉄則B64
const INF: i64 = 1 << 60;

// 経路の復元
// 終点から復元する。
fn main() {
    input!{n: usize, m: usize}
    let mut graph: Vec<Vec<(usize, i64)>> = vec![vec![]; n];
    for _ in 0..m {
        input!{a: usize, b: usize, c: i64}
        graph[a-1].push((b-1, c));
        graph[b-1].push((a-1, c));
    }
    let dist = dijkstra(0, &graph);

    // dpの復元の応用
    let mut cur_v: usize = n-1;
    let mut order: Vec<usize> = Vec::new();
    order.push(cur_v);
    while cur_v > 0 {
        // 隣接ノードを探索
        for &(next, cost) in graph[cur_v].iter() {
            // 0 -> next -> cur -> n-1
            if dist[cur_v] == dist[next] + cost {
                order.push(next);
                cur_v = next;
                // continueは避けた方がいい
                break;
            }
        }
    }
    order.reverse();
    for &v in order.iter() {
        print!("{} ", v+1);
    }
}

fn dijkstra(start: usize, graph: &Vec<Vec<(usize, i64)>>) -> Vec<i64> {
    let n: usize = graph.len();
    let mut dist: Vec<i64> = vec![INF; n];
    dist[start] = 0;
    let mut que = std::collections::VecDeque::new();
    que.push_back(start);

    while let Some(vertex) = que.pop_front() {
        for &(next, cost) in graph[vertex].iter() {
            if dist[next] > dist[vertex] + cost {
                dist[next] = dist[vertex] + cost;
                que.push_back(next);
            }
        }
    }

    return dist;
}