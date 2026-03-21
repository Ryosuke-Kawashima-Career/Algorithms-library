use ordered_float::NotNan;

fn main() {
    // max heap
    let mut maxheap = std::collections::BinaryHeap::new();
    maxheap.push(NotNan::new(x).unwrap());
    if let Some(nn) = maxheap.pop() {
        println!("{}", nn.into_inner());
    }
    // manual
    maxheap.push(Float(x));

    // min heap
    let mut minheap = std::collections::BinaryHeap::new();
    minheap.push(Reverse(NotNan::new(x).unwrap()));
    if let Some(Reverse(nn)) = minheap.pop() {
        println!("{}", nn.into_inner());
    }
    // manual
    minheap.push(MinFloat(x));
}

// Ordered Float
struct Float(f64);

impl PartialOrd for Float {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.0.partial_cmp(&other.0)
    }
}

impl Ord for Float {
    fn cmp(&self, other: &Float) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl PartialEq for Float {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl Eq for Float {}

// Reverse Ordered Float
struct MinFloat(f64);
// reverse other and self
impl PartialOrd for MinFloat {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        other.0.partial_cmp(&self.0)
    }
}
// not!!! reverse other and self
impl Ord for MinFloat {
    fn cmp(&self, other: &MinFloat) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl PartialEq for MinFloat {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl Eq for MinFloat {}