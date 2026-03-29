// A highly optimized Bipartite Union-Find (DSU) to track dynamic odd cycles and colors
struct BipartiteDSU {
    parent: Vec<usize>,
    color: Vec<i32>,
    size: Vec<[usize; 2]>,
    is_bipartite: bool,
    pub total_min_black: usize,
}

impl BipartiteDSU {
    fn new(n: usize) -> Self {
        let mut parent = vec![0; n];
        let mut size = vec![[0, 0]; n];

        for i in 0..n {
            parent[i] = i;
            // Every isolated vertex initially belongs to color "0" of its own standalone component (Black/White parity)
            size[i][0] = 1;
            size[i][1] = 0;
        }

        BipartiteDSU {
            parent,
            color: vec![0; n],
            size,
            is_bipartite: true,
            total_min_black: 0,
        }
    }

    // Path compression that dynamically ripples the color offset up relative to the root component
    fn find(&mut self, i: usize) -> (usize, i32) {
        if self.parent[i] == i {
            (i, 0)
        } else {
            let p = self.parent[i];
            let (root, c) = self.find(p);
            self.parent[i] = root;
            self.color[i] ^= c; // Update absolute offset structurally
            (root, self.color[i])
        }
    }

    // Attempt to drop an edge between independent or existing cluster components
    fn union(&mut self, i: usize, j: usize) {
        if !self.is_bipartite {
            return;
        } // Once dead, always dead computationally

        let (root_i, c_i) = self.find(i);
        let (root_j, c_j) = self.find(j);

        if root_i == root_j {
            // They are already strictly chained within the exact same component structure
            if c_i == c_j {
                // If they map identically to the same exact color relative to root, forming a 1-edge breaks bipartiteness!
                self.is_bipartite = false;
            }
        } else {
            // We want c_i ^ c_edge ^ c_j to mathematically equate to 1 (ensuring different colors relative)
            let c_edge = 1 ^ c_i ^ c_j;

            let mut ri = root_i;
            let mut rj = root_j;

            // Union By Size (strictly mapping smaller segments functionally into the massive networks)
            if self.size[ri][0] + self.size[ri][1] < self.size[rj][0] + self.size[rj][1] {
                std::mem::swap(&mut ri, &mut rj);
            }

            // Revert their previous actively tracked isolated mathematical minimum values out
            self.total_min_black -= self.size[ri][0].min(self.size[ri][1]);
            self.total_min_black -= self.size[rj][0].min(self.size[rj][1]);

            // Absorb the smaller root dimension completely into the larger mathematically
            self.parent[rj] = ri;
            self.color[rj] = c_edge;

            if c_edge == 0 {
                // Their assigned colors perfectly/identically overlap cleanly natively
                self.size[ri][0] += self.size[rj][0];
                self.size[ri][1] += self.size[rj][1];
            } else {
                // The incoming child group functionally needs their internally tracked sizes inverted
                self.size[ri][0] += self.size[rj][1];
                self.size[ri][1] += self.size[rj][0];
            }

            // Register their shiny new merged Minimum constraint back permanently
            self.total_min_black += self.size[ri][0].min(self.size[ri][1]);
        }
    }
}
