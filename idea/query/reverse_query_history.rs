use proconio::{input, marker::Chars};
// ABC411D
// Q. Query1: pc from server Query2: Add strings Query3: server from pc
// Q. What is the string of the server at Query=q
// A. Reverse the queries: クエリの反転
// A. Trace Back: 歴史をさかのぼる感じ
// Server = PC0
fn main() {
    input!{_n: usize, q: usize}
    let queries: Vec<(usize, usize, Vec<char>)> = read_queries(q);

    // Query Reverse: Initial one is the server
    let mut cur_pc: usize = 0;
    let mut cur_string_reversed: Vec<char> = Vec::new();
    for (query_type, p, s) in queries.iter().rev() {
        if *query_type == 1 {
            if cur_pc == *p {
                cur_pc = 0;
            }
        } else if *query_type == 2 {
            if cur_pc == *p {
                cur_string_reversed.extend(s);
            }
        } else {
            if cur_pc == 0 {
                cur_pc = *p;
            }
        }
    }
    let answer: String = cur_string_reversed.iter().rev().collect();
    println!("{}", answer);
}

fn read_queries(q: usize) -> Vec<(usize, usize, Vec<char>)> {
    let mut queries = Vec::new();
    for _ in 0..q {
        input!{query_type: usize}
        if query_type == 1 {
            input!{p: usize}
            queries.push((query_type, p, vec![]));
        } else if query_type == 2 {
            input!{p: usize, s: Chars}
            let s_reversed: Vec<char> = s.into_iter().rev().collect::<Vec<char>>();
            queries.push((query_type, p, s_reversed));
        } else if query_type == 3 {
            input!{p: usize}
            queries.push((query_type, p, vec![]));
        }
    }
    return queries;
}