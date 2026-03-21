use std::io::{stdin, stdout, BufRead, BufReader, Write};
use proconio::{input, marker::Chars, source::line::LineSource};
// abc337e
// 二進法を利用することで対象を特定する
fn main() {
    // interactive input
    let stdin = stdin();
    let mut source = LineSource::new(BufReader::new(stdin.lock()));
    stdout().flush().unwrap();
    input! {
        from &mut source,
        n: usize,
    }

    // m = log2(n) <=> 2^m = n
    // (power, exponential)
    let (m, _exp) = next_pow2(n);

    // 二進法からbitの演算をし，質問を作る
    let questions = binary_distribute(m, n);

    let reply = query(&questions, &mut source);

    // 二進法のbitの演算orを使い，特定する
    let ans = binary_identify(&reply);
    
    // 0indexed -> 1indexed
    println!("{}", ans+1);
}

fn binary_distribute(m: usize, n: usize) -> Vec<Vec<usize>> {
    let mut questions = Vec::new();
    // m == power: player(i)
    for i in 0..m {
        let mut juices = Vec::new();
        // n == exponential: juice(j)
        for j in 0..n {
            // 2^m == n: iが桁, jがbit
            if j >> i & 1 == 1 {
                juices.push(j);
            }
        }
        questions.push(juices);
    }
    return questions;
}

// 上の桁から計算する: (0..m).rev()
fn binary_identify(bit: &Vec<usize>) -> usize {
    let m: usize =  bit.len();
    let mut res: usize = 0;

    for i in 0..m {
        if bit[i] == 1 {
            res |= 1 << i;
        }
    }

    return res;
}

// 2 ^ m >= n
#[allow(dead_code)]
fn next_pow2(n: usize) -> (usize, usize) {
    let mut m: usize = 0;
    while (1 << m) < n {
        m += 1;
    }
    return (m, 1 << m);
}
#[allow(dead_code)]
fn next_power2(n: usize) -> (usize, usize) {
    // 2 ^ pow == exp
    let mut exp: usize = 1;
    let mut pow: usize = 0;
    while exp < n {
        pow += 1;
        exp <<= 1;
    }
    return (pow, exp);
}

// interactive output + input 
fn query<R: BufRead>(questions: &Vec<Vec<usize>>, source: &mut LineSource<R>) -> Vec<usize> {
    let m: usize = questions.len();
    println!("{}", m);
    for i in 0..m {
        let k: usize = questions[i].len();
        print!("{}", k);
        for j in 0..k {
            // 0indexed -> 1indexed
            print!(" {}", questions[i][j]+1);
        }
        println!("");
    }
    stdout().flush().unwrap();
    input! {
        from source,
        s: Chars,
    }
    let y: Vec<usize> = s.iter().map(|&x| x as usize - '0' as usize).collect();
    return y;
}