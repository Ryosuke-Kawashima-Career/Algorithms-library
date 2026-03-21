// 幅優先探索でv0と接続の頂点を求める
fn bfs(v0: usize, graph: &Vec<Vec<usize>>) -> Vec<usize> {
    let n: usize = graph.len();
    // v0に到達可能な頂点を格納する
    let mut reachables: Vec<usize> = Vec::new();
    let mut que = std::collections::VecDeque::new();
    let mut seen: Vec<bool> = vec![false; n];
    seen[v0] = true;
    que.push_back(v0);
    
    while let Some(v) = que.pop_front() {
        reachables.push(v);
        for &next in graph[v].iter() {
            // seenを変更してからqueに入れることでqueの頂点のダブりを防ぐ
            if !seen[next] {
                // the place of seen[node]=true is critical !!!
                seen[next] = true;
                que.push_back(next);
            }
        }
    }

    return reachables;
}