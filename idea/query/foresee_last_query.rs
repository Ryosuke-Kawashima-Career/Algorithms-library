use proconio::{input, marker::Chars};
// abc314D
// t!=1のクエリのうち，意味のあるクエリは最後のもの
// 最後のクエリが最後の状態(the last query is the last state)
// クエリを先読みしてt=1の最後のクエリのindexを求めておき，それ以外のt=1のクエリは無視しながらそのままシミュレーション
fn main() {
    input!{n: usize, mut s: Chars, q: usize, queries: [(usize, usize, char); q]}
    // 最後のクエリのindex(query != 1)
    let mut last_query: usize = n;
    for i in 0..q {
        if queries[i].0 != 1 {
            last_query = i;
        }
    }

    // simulation
    for query in 0..q {
        let (query_type, x, c) = queries[query];
        if query_type == 1 {
            s[x-1] = c;
        } else if query_type == 2 {
            if query == last_query {
                for i in 0..n {
                    s[i] = to_lowercase(s[i]);
                }
            }
        } else {
            if query == last_query {
                for i in 0..n {
                    s[i] = to_uppercase(s[i]);
                }
            }
        }
    }

    for i in 0..n {
        print!("{}", s[i]);
    }
    println!("");
}

fn to_lowercase(c: char) -> char {
    if c as u8 - ('a' as u8) < 26 {
        return c;
    }
    return (c as u8 - 'A' as u8 + 'a' as u8) as char;
}

fn to_uppercase(c: char) -> char {
    if c as u8 - ('a' as u8) >= 26 {
        return c;
    }
    return (c as u8 - 'a' as u8 + 'A' as u8) as char;
}