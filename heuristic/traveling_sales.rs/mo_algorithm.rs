use proconio::input;
// ABC448F
// Q. Traveling Salesperson Problem (Hamiltonian Cycle: Each node just once)
/* A. Mo's algorithm
1. Bucket the X-Axis: Imagine drawing vertical lines across the 2D plane to create columns or "buckets" of width exactly $B$.
2. Assign Points to Buckets: Every point is assigned to a bucket based on its X-coordinate `(x / B)`.
3. Sort by Bucket: We visit all points in Bucket 0, then move to Bucket 1, Bucket 2, etc. This guarantees that our horizontal (X-axis) jumps are typically very small (bounded by $B$) because we fully exhaust a tight column before moving to the next.
4. Snake-like Y-Axis Traversal: Inside a bucket, we just sweep the Y-axis. But if we sweep from Y=0 to Y=Max inside Bucket 0, then jump into Bucket 1 and start from Y=0 again, we make a massive diagonal jump! To fix this, we sweep UP (ascending Y) for even buckets, and sweep DOWN (descending Y) for odd buckets. This creates a smooth "snake" or "zigzag" pattern across the plane.
5. Start at Node 1: This zigzag pattern connects all nodes optimally into an unclosed line. But a TSP is a connected ring/cycle. We connect the very last point to the very first point to complete the circle. Then, since we logically must start the journey at Node 1, we just start iterating and printing from Node 1 instead of the absolute top-left of the sorting array. */
#[derive(Clone, Copy, Debug)]
struct Point {
    id: usize,
    x: i64,
    y: i64,
}

fn main() {
    input! {
        n: usize,
        xy: [(i64, i64); n],
    }
    let mut nodes: Vec<Point> = Vec::with_capacity(n);
    for i in 0..n {
        let (x, y) = xy[i];
        nodes.push(Point { id: i + 1, x, y });
    }
    mo_algorithm(&mut nodes);
    print_cycle(&nodes);
}

fn mo_algorithm(nodes: &mut Vec<Point>) {
    let n: usize = nodes.len();
    let max_x = nodes.iter().map(|p| p.x).max().unwrap();
    let max_y = nodes.iter().map(|p| p.y).max().unwrap();
    // Mathematically optimal Bucket Size (B)
    let mut bucket_size = ((max_x as f64 * max_y as f64 / n as f64).sqrt()) as i64;
    if bucket_size <= 0 {
        bucket_size = 1;
    }

    // Sort the plane like Mo's Algorithm
    nodes.sort_unstable_by(|p1, p2| {
        let bucket_x1 = p1.x / bucket_size;
        let bucket_x2 = p2.x / bucket_size;
        if bucket_x1 != bucket_x2 {
            bucket_x1.cmp(&bucket_x2) // Sort by X-Bucket
        } else {
            // Sort by Y-coordinate, alternating directions based on the bucket
            if bucket_x1 % 2 == 0 {
                p1.y.cmp(&p2.y)
            } else {
                p2.y.cmp(&p1.y)
            }
        }
    });
}

fn print_cycle(nodes: &Vec<Point>) {
    let n: usize = nodes.len();
    // The tour is a cycle. Find where node 1 is, and start from there.
    let start_idx = nodes.iter().position(|p| p.id == 1).unwrap();

    for i in 0..n {
        let idx = (start_idx + i) % n;
        print!("{} ", nodes[idx].id);
    }
    println!();
}
