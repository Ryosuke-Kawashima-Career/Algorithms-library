const INF: i64 = 1 << 60;

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