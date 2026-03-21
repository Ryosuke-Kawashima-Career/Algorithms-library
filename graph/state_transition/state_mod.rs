use proconio::input;
use std::collections::VecDeque;
// ABC446E
// Q. s_n = (a * s_{n-1} + b * s_{n-2}) % m
// A. Calculate the state of mod m and explore the graph by DFS or BFS.
// The state is defined as (s_{n-2}, s_{n-1}).
// The transition is defined as (s_{n-2}, s_{n-1}) -> (s_{n-1}, s_n).
// The goal is to find the number of states that do not contain 0.
fn main() {
    input! {m: usize, a: usize, b: usize}
    let graph = make_state_graph(m, a, b);
    let state_seen: Vec<bool> = bfs(m, &graph);
    let mut count: usize = 0;
    for x in 0..m {
        for y in 0..m {
            let state: usize = m * x + y;
            if !state_seen[state] {
                count += 1;
            }
        }
    }
    println!("{}", count);
}

fn make_state_graph(m: usize, a: usize, b: usize) -> Vec<Vec<usize>> {
    /* Calculates the transition of the states
    (s_{n-2}, s_{n-1}) -> (s_{n-1}, s_n)
    where s_n = (a * s_{n-1} + b * s_{n-2}) % m
    */
    let num_states: usize = m * m;
    let mut state_graph: Vec<Vec<usize>> = vec![vec![]; num_states];
    for x in 0..m {
        for y in 0..m {
            let state: usize = m * x + y;
            let next_x: usize = y;
            let next_y: usize = (a * y + b * x) % m;
            let next_state: usize = m * next_x + next_y;
            state_graph[next_state].push(state);
        }
    }
    state_graph
}

fn bfs(m: usize, graph: &Vec<Vec<usize>>) -> Vec<bool> {
    let num_states: usize = m * m;
    let mut state_seen: Vec<bool> = vec![false; num_states];
    let mut que: VecDeque<usize> = VecDeque::new();
    for i in 0..m {
        let state1: usize = m * i + 0;
        let state2: usize = m * 0 + i;
        state_seen[state1] = true;
        state_seen[state2] = true;
        que.push_back(state1);
        que.push_back(state2);
    }
    while let Some(state) = que.pop_front() {
        for &next_state in graph[state].iter() {
            if !state_seen[next_state] {
                state_seen[next_state] = true;
                que.push_back(next_state);
            }
        }
    }
    state_seen
}
