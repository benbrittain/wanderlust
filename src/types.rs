#[derive(Debug, Copy, Clone)]
pub struct Point {
    pub x: f64,
    pub y: f64,
    pub timestamp: i64
}

impl PartialEq for Point {
    fn eq(&self, other: &Point) -> bool {
        self.x == other.x && self.y == other.y
        // not timestamp, otherwise clustering won't work
    }
}

impl Point {
    pub fn new(x: f64, y: f64, ts: i64) -> Point {
        Point {
            x: x,
            y: y,
            timestamp: ts
        }
    }
}

