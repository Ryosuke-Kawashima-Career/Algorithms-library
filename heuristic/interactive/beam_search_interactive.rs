use proconio::{input, source::line::LineSource};
use std::collections::VecDeque;
use std::io::{stdin, stdout, BufRead, BufReader, Write};

const MOVES: [(usize, usize); 5] = [
    (1usize.wrapping_neg(), 0), // Up -> dx = -1
    (0, 1usize.wrapping_neg()), // Left -> dy = -1
    (0, 0),                     // Stay
    (0, 1),                     // Right -> dy = 1
    (1, 0),                     // Down -> dx = 1
];

// Random behaviors
pub struct Xorshift {
    state: u32,
}

impl Xorshift {
    pub fn new(seed: u32) -> Xorshift {
        Xorshift { state: seed }
    }

    pub fn next(&mut self) -> u32 {
        let mut x: u32 = self.state;
        x ^= x << 13;
        x ^= x >> 17;
        x ^= x << 5;
        self.state = x;
        self.state
    }

    pub fn rangef(&mut self, min_num: f64, max_num: f64) -> f64 {
        let rng = (self.next() as f64) / (u32::MAX as f64);
        min_num + (rng * (max_num - min_num))
    }
}

#[derive(Debug, Clone)]
struct Input {
    n_board_size: usize,
    m_player: usize,
    t_turn: usize,
    u_max_level: usize,
    values_cells: Vec<Vec<usize>>,
    init_cells: Vec<(usize, usize)>,
}

impl Input {
    fn new<R: BufRead>(source: &mut LineSource<R>) -> Self {
        input! {
            from source,
            n_board_size: usize, m_player: usize, t_turn: usize, u_max_level: usize,
            values_cells: [[usize; n_board_size]; n_board_size],
            init_cells: [(usize, usize); m_player],
        }
        Self {
            n_board_size,
            m_player,
            t_turn,
            u_max_level,
            values_cells,
            init_cells,
        }
    }
}

#[derive(Debug, Clone)]
struct State {
    destinations: Vec<(usize, usize)>,
    scores: Vec<usize>,
    positions: Vec<(usize, usize)>,
    owners: Vec<Vec<isize>>, // -1 means no owner
    levels: Vec<Vec<usize>>,
    cells_players: Vec<Vec<(usize, usize)>>,
}

impl State {
    fn init(input: &Input) -> Self {
        let mut owners: Vec<Vec<isize>> = vec![vec![-1; input.n_board_size]; input.n_board_size];
        let mut levels: Vec<Vec<usize>> = vec![vec![0; input.n_board_size]; input.n_board_size];
        let positions: Vec<(usize, usize)> = input.init_cells.clone();
        let mut scores: Vec<usize> = vec![0; input.m_player];
        let destinations: Vec<(usize, usize)> = input.init_cells.clone();
        let mut cells_players: Vec<Vec<(usize, usize)>> = vec![vec![]; input.m_player];

        for owner in 0..input.m_player {
            let (r, c) = input.init_cells[owner];
            owners[r][c] = owner as isize;
            levels[r][c] = 1;
            scores[owner] = input.values_cells[r][c];
            cells_players[owner].push((r, c));
        }

        Self {
            destinations,
            scores,
            positions,
            owners,
            levels,
            cells_players,
        }
    }
}

fn interactive_io<R: BufRead>(
    source: &mut LineSource<R>,
    tx0: usize,
    ty0: usize,
    input: &Input,
    state: &mut State,
) {
    println!("{} {}", tx0, ty0);
    stdout().flush().unwrap();

    input! {
        from source,
        txy_chosen_by_players: [(usize, usize); input.m_player],
        each_player_positions: [(usize, usize); input.m_player],
        owners_of_cells: [[isize; input.n_board_size]; input.n_board_size],
        levels_of_cells: [[usize; input.n_board_size]; input.n_board_size],
    }

    state.destinations = txy_chosen_by_players;
    state.positions = each_player_positions;
    state.owners = owners_of_cells;
    state.levels = levels_of_cells;

    let mut new_scores: Vec<usize> = vec![0; input.m_player];
    state.cells_players = vec![vec![]; input.m_player];

    for i in 0..input.n_board_size {
        for j in 0..input.n_board_size {
            if state.owners[i][j] != -1 {
                let owner = state.owners[i][j] as usize;
                new_scores[owner] += input.values_cells[i][j] * state.levels[i][j];
                state.cells_players[owner].push((i, j));
            }
        }
    }
    state.scores = new_scores;
}

fn main() {
    let stdin = stdin();
    let mut source = LineSource::new(BufReader::new(stdin.lock()));
    let input = Input::new(&mut source);
    let mut state = State::init(&input);

    for _turn in 0..input.t_turn {
        let (tx0, ty0) = decide_destination_beam_search(&input, &state);
        interactive_io(&mut source, tx0, ty0, &input, &mut state);
    }
}

/// Computes the valid destinations for player 0.
fn get_valid_destinations(
    pos: (usize, usize),
    owners: &Vec<Vec<isize>>,
    positions: &Vec<(usize, usize)>,
    n: usize,
    m_player: usize,
) -> Vec<(usize, usize)> {
    // BFS to find reachable territory
    let mut reachable = vec![vec![false; n]; n];
    let mut queue = VecDeque::new();

    if owners[pos.0][pos.1] == 0 {
        queue.push_back(pos);
        reachable[pos.0][pos.1] = true;
    }

    let mut reachable_cells = Vec::new();

    while let Some((x, y)) = queue.pop_front() {
        reachable_cells.push((x, y));

        for &(dx, dy) in &MOVES[..4] {
            let nx = x.wrapping_add(dx);
            let ny = y.wrapping_add(dy);

            if nx < n && ny < n && !reachable[nx][ny] {
                if owners[nx][ny] == 0 {
                    reachable[nx][ny] = true;
                    queue.push_back((nx, ny));
                }
            }
        }
    }

    let mut valid_dest_mask = vec![vec![false; n]; n];
    for &(rx, ry) in &reachable_cells {
        valid_dest_mask[rx][ry] = true;
        for &(dx, dy) in &MOVES[..4] {
            let nx = rx.wrapping_add(dx);
            let ny = ry.wrapping_add(dy);
            if nx < n && ny < n {
                valid_dest_mask[nx][ny] = true;
            }
        }
    }

    let mut valid_destinations = Vec::new();
    for i in 0..n {
        for j in 0..n {
            if valid_dest_mask[i][j] {
                let mut has_other_piece = false;
                for p in 1..m_player {
                    if positions[p] == (i, j) {
                        has_other_piece = true;
                        break;
                    }
                }
                if !has_other_piece {
                    valid_destinations.push((i, j));
                }
            }
        }
    }

    valid_destinations
}

#[derive(Clone, Debug)]
struct BeamState {
    owners: Vec<Vec<isize>>,
    levels: Vec<Vec<usize>>,
    my_pos: (usize, usize),
    first_move: Option<(usize, usize)>,
    score: f64,
}

impl BeamState {
    fn new(state: &State) -> Self {
        Self {
            owners: state.owners.clone(),
            levels: state.levels.clone(),
            my_pos: state.positions[0],
            first_move: None,
            score: 0.0,
        }
    }

    fn apply_move(&mut self, dest: (usize, usize), input: &Input) {
        let (tx, ty) = dest;
        let owner = self.owners[tx][ty];

        if owner == -1 {
            // Occupation
            self.owners[tx][ty] = 0;
            self.levels[tx][ty] = 1;
        } else if owner == 0 {
            // Reinforcement
            if self.levels[tx][ty] < input.u_max_level {
                self.levels[tx][ty] += 1;
            }
        } else {
            // Attack
            if self.levels[tx][ty] == 1 {
                self.owners[tx][ty] = 0;
                self.levels[tx][ty] = 1;
            } else {
                self.levels[tx][ty] -= 1;
                // Piece would be removed here strictly speaking, but for beam search approximation
                // we'll assume we keep attacking or re-enter.
            }
        }

        self.my_pos = dest;
    }

    fn evaluate(&mut self, input: &Input) {
        let mut board_score = 0.0;
        let mut potential_score = 0.0;
        let n = input.n_board_size;

        for i in 0..n {
            for j in 0..n {
                if self.owners[i][j] == 0 {
                    board_score += (input.values_cells[i][j] * self.levels[i][j]) as f64;
                    // Proximity bonus for expansion
                    for &(dx, dy) in &MOVES[..4] {
                        let nx = i.wrapping_add(dx);
                        let ny = j.wrapping_add(dy);
                        if nx < n && ny < n {
                            if self.owners[nx][ny] == -1 {
                                potential_score += input.values_cells[nx][ny] as f64 * 0.1;
                            } else if self.owners[nx][ny] > 0 {
                                potential_score += input.values_cells[nx][ny] as f64 * 0.5;
                                // High priority target
                            }
                        }
                    }
                }
            }
        }

        self.score = board_score + (potential_score * 0.1); // Tune alpha
    }
}

/// Decides the destination using a Beam Search strategy.
fn decide_destination_beam_search(input: &Input, state: &State) -> (usize, usize) {
    let mut rng = Xorshift::new(42);
    let beam_width = 20;
    let max_depth = 4;

    let initial_state = BeamState::new(state);
    let mut beam = vec![initial_state.clone()];

    for depth in 0..max_depth {
        let mut next_beam = Vec::new();

        for b_state in &beam {
            let valid_dests = get_valid_destinations(
                b_state.my_pos,
                &b_state.owners,
                &state.positions, // AIs are assumed stationary for simple lookahead
                input.n_board_size,
                input.m_player,
            );

            // If we have no valid moves (shouldn't happen), push self
            if valid_dests.is_empty() {
                next_beam.push(b_state.clone());
                continue;
            }

            for dest in valid_dests {
                let mut next_state = b_state.clone();
                next_state.apply_move(dest, input);
                next_state.evaluate(input);

                // Keep track of the first move
                if depth == 0 {
                    next_state.first_move = Some(dest);
                }

                // Add random tie-breaker
                next_state.score += rng.rangef(0.0, 1e-6);

                next_beam.push(next_state);
            }
        }

        // Sort decreasing (largest score first)
        next_beam.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap());

        if next_beam.len() > beam_width {
            next_beam.truncate(beam_width);
        }

        beam = next_beam;
    }

    if let Some(best_state) = beam.first() {
        if let Some(mv) = best_state.first_move {
            return mv;
        }
    }

    state.positions[0]
}
