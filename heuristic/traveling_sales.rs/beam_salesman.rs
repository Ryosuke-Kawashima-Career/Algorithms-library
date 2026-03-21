use proconio::input;
use proconio::derive_readable;
// 鉄則A46
#[derive_readable]
#[derive(Debug, Copy, Clone)]
struct Point {
    x: f64, y: f64
}
impl Point {
    fn new(pos_x: f64, pos_y: f64) -> Self {
        Self{x: pos_x, y: pos_y}
    }
}
// beam search: 上位k個の候補の遷移を計算する
// beam search: (greedy search + exhaustive search) / 2
fn main() {
    input!{n: usize, xy: [Point; n]}
    let mut bs = BeamSearch::new(n, 150);
    // beam search
    for _ in 0..n {
        bs.next_step(&xy);
    }
    let path = bs.get_best_moves();

    for &v in path.iter() {
        println!("{}", v+1);
    }
}

// なるべく関数で実装する(クロージャーでは所有権の問題が発生する)
fn calc_dist(i: usize, j: usize, xy: &Vec<Point>) -> f64 {
    let x_sq = (xy[j].x - xy[i].x) * (xy[j].x - xy[i].x);
    let y_sq = (xy[j].y - xy[i].y) * (xy[j].y - xy[i].y);
    (x_sq + y_sq).sqrt()
}

// beam search
#[derive(Debug, Clone)]
struct Node {
    score: f64,
    used: Vec<bool>,
    cur_node: usize,
    last_rank: usize,
}
impl Node {
    fn new(n: usize) -> Self {
        Self {
            score: 0.0,
            used: vec![false; n],
            cur_node: 0,
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

    fn next_step(&mut self, graph: &Vec<Point>) {
        let n: usize = graph.len();
        let phase: usize = self.beams.len();
        let last_beam: &Vec<Node> = self.beams.last().unwrap();
        // within beamsize
        let size: usize = last_beam.len().min(self.beam_size);
        let mut new_beam: Vec<Node> = Vec::new();

        for i in 0..size {
            // only the start 0 remains
            if phase == n {
                let mut last_used = last_beam[i].used.clone();
                let last_score: f64 = last_beam[i].score - calc_dist(last_beam[i].cur_node, 0, graph);
                last_used[0] = true;
                new_beam.push(Node {
                    score: last_score,
                    used: last_used,
                    cur_node: 0,
                    last_rank: i,
                });
                continue;
            }

            // score is - distance
            for next_node in 1..n {
                let mut next_used = last_beam[i].used.clone();
                if next_used[next_node] {
                    continue;
                }
                let next_score: f64 = last_beam[i].score - calc_dist(last_beam[i].cur_node, next_node, graph);
                next_used[next_node] = true;
                new_beam.push(Node {
                    score: next_score,
                    used: next_used,
                    cur_node: next_node,
                    last_rank: i,
                });
            }
        }
        new_beam.sort_by(|x, y| y.score.partial_cmp(&x.score).unwrap());
        self.beams.push(new_beam);
    }

    fn get_best_moves(&self) -> Vec<usize> {
        let mut moves: Vec<usize> = Vec::new();
        let mut rank: usize = 0;
        // 過去をさかのぼる
        for step in (0..self.beams.len()).rev() {
            moves.push(self.beams[step][rank].cur_node);
            rank = self.beams[step][rank].last_rank;
        }
        moves.reverse();
        moves
    }
}