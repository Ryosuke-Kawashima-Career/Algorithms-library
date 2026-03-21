struct Point {
    x: i64,
    y: i64,
}
impl Point {
    fn new(x: i64, y: i64) -> Point {
        Point { x, y }
    }
}
struct LineSegment {
    p1: Point,
    p2: Point,
}
impl LineSegment {
    fn new(p1: Point, p2: Point) -> LineSegment {
        Line { p1, p2 }
    }
    fn is_crossed_with(&self, another: &LineSegment) -> bool {
        let s: i64 = (self.p2.x - self.p1.x) * (another.p2.y - another.p1.y)
            - (self.p2.y - self.p1.y) * (another.p2.x - another.p1.x);
        let t: i64 = (self.p2.x - self.p1.x) * (another.p1.y - self.p1.y)
            - (self.p2.y - self.p1.y) * (another.p1.x - self.p1.x);
        let u: i64 = (another.p2.x - another.p1.x) * (self.p1.y - another.p1.y)
            - (another.p2.y - another.p1.y) * (self.p1.x - another.p1.x);
        let v: i64 = (another.p2.x - another.p1.x) * (self.p2.y - another.p1.y)
            - (another.p2.y - another.p1.y) * (self.p2.x - another.p1.x);
        return s * t < 0 && u * v < 0;
    }
}
