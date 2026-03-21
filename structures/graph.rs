#[derive(Copy, Clone)]
struct Edge {
    to: usize, weight: i64
}

impl Edge {
    fn new(to: usize, weight: i64) -> Self {
        Edge{to: to, weight: weight}
    }
}