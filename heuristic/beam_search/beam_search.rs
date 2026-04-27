use proconio::{input, marker::Usize1};
// 鉄則A49
// beam search: 上位k個の候補の遷移を計算する
// beam search: (greedy search + exhaustive search) / 2
// O(beam size)
fn main() {
    input!{t: usize, pqr: [[Usize1; 3]; t]}
    // A: +1, B: -1
    let mut bs = BeamSearch::new(20, 20000);
    // 貪欲法の拡張: 上位kの遷移を計算する
    for idx in 0..t {
        bs.next_step(&pqr[idx]);
    }
    let operations = bs.get_best_moves();
    // print result
    for &operation in operations.iter() {
        match operation {
            Move::A => println!("A"),
            Move::B => println!("B"),
            Move::None => println!("No Move!"),
        }
    }
}

// 絶対値の和を評価関数とする: コストに-1を掛けることでスコアにする
fn evaluate_all(x: &Vec<i64>) -> i64 {
    let cost: i64 = x.iter().map(|&val| val.abs()).sum();
    -cost
}

// beam search library
#[derive(Debug, Copy, Clone)]
enum Move {
    A,
    B,
    None,
}

#[derive(Debug, Clone)]
struct Node {
    score: i64,
    states: Vec<i64>,
    last_move: Move,
    last_rank: usize,
}

impl Node {
    fn new(n: usize) -> Self {
        Self {
            score: 0,
            states: vec![0; n],
            last_move: Move::None,
            last_rank: 0,
        }
    }
}

#[derive(Debug, Clone)]
struct BeamSearch {
    beams: Vec<Vec<Node>>,
    beam_size: usize,
}

impl BeamSearch {
    fn new(n: usize, k: usize) -> Self {
        Self {
            beams: vec![vec![Node::new(n); 1]; 1],
            beam_size: k,
        }
    }

    // 次の操作を行い，ノードの遷移を計算する
    fn next_step(&mut self, indexes: &Vec<usize>) {
        let last_beam: &Vec<Node> = self.beams.last().unwrap();
        // beam_sizeよりも順位が大きいものの探索をやめる
        let size: usize = last_beam.len().min(self.beam_size);
        let mut new_beam: Vec<Node> = Vec::new();

        for i in 0..size {
            let states_a: Vec<i64> = operation(&last_beam[i].states, indexes, Move::A);
            let score_a: i64 = evaluate_all(&states_a);
            new_beam.push(Node {
                score: last_beam[i].score + score_a,
                states: states_a,
                last_move: Move::A,
                last_rank: i,
            });
            let states_b: Vec<i64> = operation(&last_beam[i].states, indexes, Move::B);
            let score_b: i64 = evaluate_all(&states_b);
            new_beam.push(Node {
                score: last_beam[i].score + score_b,
                states: states_b,
                last_move: Move::B,
                last_rank: i,
            });
        }
        new_beam.sort_by(|x, y| y.score.cmp(&x.score));
        self.beams.push(new_beam);
    }

    fn get_best_moves(&self) -> Vec<Move> {
        let mut moves: Vec<Move> = Vec::new();
        let mut rank: usize = 0;
        // 過去をさかのぼる
        for step in (1..self.beams.len()).rev() {
            moves.push(self.beams[step][rank].last_move.clone());
            rank = self.beams[step][rank].last_rank;
        }
        moves.reverse();
        moves
    }
}

// functions
fn operation(x: &Vec<i64>, indexes: &Vec<usize>, operation: Move) -> Vec<i64> {
    let mut new_x = x.clone();
    let n: usize = indexes.len();
    for i in 0..n {
        match operation {
            Move::A => {
                new_x[indexes[i]] += 1;
            },
            Move::B => {
                new_x[indexes[i]] -= 1;
            },
            Move::None => {
                println!("Invalid Operation!");
            }
        }
    }
    new_x
}