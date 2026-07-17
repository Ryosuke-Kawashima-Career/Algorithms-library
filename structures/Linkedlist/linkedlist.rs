struct Node {
    /* Use index instead of pointer */
    id: usize,
    next: Option<usize>,
    prev: Option<usize>,
}

struct LinkedList {
    nodes: Vec<Node>,
}

impl LinkedList {
    fn new() -> Self {
        Self { nodes: Vec::new() }
    }

    fn create_node(&mut self, id: usize) -> usize {
        /*Add a new node and return the new index*/
        let idx = self.nodes.len();
        self.nodes.push(Node {
            id,
            next: None,
            prev: None,
        });
        idx
    }

    fn connect(&mut self, from_idx: usize, to_idx: usize) {
        /*Connect from_idx to to_idx*/
        if let Some(old_next) = self.nodes[from_idx].next {
            self.nodes[old_next].prev = None;
        }

        if let Some(old_prev) = self.nodes[to_idx].prev {
            self.nodes[old_prev].next = None;
        }
        self.nodes[from_idx].next = Some(to_idx);
        self.nodes[to_idx].prev = Some(from_idx);
    }

    fn count_len(&self, start_idx: usize) -> usize {
        /*Count the length from start_idx to the end*/
        let mut count = 1;
        let mut current = start_idx;
        while let Some(next) = self.nodes[current].next {
            count += 1;
            current = next;
        }
        count
    }
}
