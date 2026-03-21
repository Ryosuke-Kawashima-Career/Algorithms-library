use proconio::input;
const INF: usize = 1 << 60;
// 典型3
// 木の中で最も離れた点の組み合わせ: 木の直径
fn main() {
    input!{n: usize}
    let mut tree: Vec<Vec<usize>> = vec![vec![]; n];
    for _ in 1..n {
        input!{a: usize, b: usize}
        tree[a-1].push(b-1);
        tree[b-1].push(a-1);
    }

    // 適当な頂点uを選ぶ。
    let dist_from_0: Vec<usize> = bfs(0, &tree);
    // uから最も遠い頂点vを選ぶ。
    let farthest_from_0: usize = argmax(&dist_from_0);
    let dist_farthest: Vec<usize> = bfs(farthest_from_0, &tree);
    // vから最も遠い頂点wを選ぶ。
    let diameter: usize = *dist_farthest.iter().max().unwrap();

    println!("{}", diameter+1);
}

fn bfs(start: usize, graph: &Vec<Vec<usize>>) -> Vec<usize> {
    let n: usize = graph.len();
    let mut dist: Vec<usize> = vec![INF; n];
    dist[start] = 0;
    let mut que = std::collections::VecDeque::new();
    que.push_back(start);

    while let Some(vertex) = que.pop_front() {
        for &next in graph[vertex].iter() {
            if dist[next] > dist[vertex] + 1 {
                dist[next] = dist[vertex] + 1;
                que.push_back(next);
            }
        }
    }

    return dist;
}

// 値が最も大きいindexを返す。
fn argmax(vector: &Vec<usize>) -> usize {
    let n: usize = vector.len();
    let mut max_index: usize = n;
    let mut max_dist: usize = 0;

    for i in 0..n {
        if max_dist < vector[i] {
            max_dist = vector[i];
            max_index = i;
        }
    }

    return max_index;
}