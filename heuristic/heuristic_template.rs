use proconio::input;

const INF: usize = 1_000_000;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Move {
    Up,
    Down,
    Left,
    Right,
}

impl Move {
    pub fn to_char(self) -> char {
        match self {
            Move::Up => 'U',
            Move::Down => 'D',
            Move::Left => 'L',
            Move::Right => 'R',
        }
    }
}

// (dx, dy, Move) corresponding to Up, Down, Left, Right
pub const DIRS: [(isize, isize, Move); 4] = [
    (-1, 0, Move::Up),
    (1, 0, Move::Down),
    (0, -1, Move::Left),
    (0, 1, Move::Right),
];

/// Data structure for holding static inputs given by the problem
#[derive(Debug, Clone)]
pub struct Input {
    pub n: usize,
    pub m: usize,
    pub grid: Vec<Vec<usize>>,
    pub target_colors: Vec<usize>,
    // Add other problem-specific constants here
}

/// Data structure for representing a mutable state during the search
#[derive(Debug, Clone)]
pub struct State {
    pub moves: Vec<Move>,
    pub score: i64,
    // Add dynamic game state variables (e.g., player position, items collected, etc.)
}

impl State {
    /// Initializes the starting state based on the input
    pub fn new(input: &Input) -> Self {
        State {
            moves: Vec::new(),
            score: 0,
        }
    }

    /// Evaluates the goodness of the current state
    /// Often called `eval` or `score`
    pub fn evaluate(&self, input: &Input) -> i64 {
        // Compute the heuristic evaluation
        self.score
    }

    /// Returns a list of valid moves from the current state
    pub fn get_valid_moves(&self, input: &Input) -> Vec<Move> {
        // Return valid moves based on problem rules
        vec![]
    }

    /// Applies a move, modifying the state and appending to history
    pub fn apply_move(&mut self, m: Move, input: &Input) {
        // Update game state logic here

        // Update the score incrementally if possible
        // self.score += ...;

        self.moves.push(m);
    }

    /// Reverts the last move, useful for Simulated Annealing or DFS/Backtracking
    pub fn revert_move(&mut self, m: Move, input: &Input) {
        // Revert game state logic here

        self.moves.pop();
    }
}

/// The optimal policy function / main solver entrypoint
/// Can be implemented using Greedy, Beam Search, or Simulated Annealing
pub fn solve(input: &Input) -> State {
    let mut state = State::new(input);

    // TODO: Implement the main heuristic algorithm
    // Example skeleton for a Greedy approach:
    //
    // loop {
    //     let valid_moves = state.get_valid_moves(input);
    //     if valid_moves.is_empty() {
    //         break;
    //     }
    //
    //     let mut best_score = std::i64::MIN;
    //     let mut best_move = None;
    //
    //     for m in valid_moves {
    //         state.apply_move(m, input);
    //         let current_score = state.evaluate(input);
    //         if current_score > best_score {
    //             best_score = current_score;
    //             best_move = Some(m);
    //         }
    //         state.revert_move(m, input);
    //     }
    //
    //     if let Some(m) = best_move {
    //         state.apply_move(m, input);
    //     } else {
    //         break;
    //     }
    // }

    state
}

fn main() {
    // Parse expected input format utilizing proconio
    input! {
        n: usize,
        m: usize,
        target_colors: [usize; m],
        grid: [[usize; n]; n],
    }

    let input_data = Input {
        n,
        m,
        grid,
        target_colors,
    };

    let final_state = solve(&input_data);

    // Output sequence of moves or other required format
    for each_move in final_state.moves {
        print!("{}", each_move.to_char());
    }
    println!();
}
