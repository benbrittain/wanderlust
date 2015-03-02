#[derive(Debug, Copy, Clone)]
pub struct Point {
    x: f64,
    y: f64
}
impl PartialEq for Point {
    fn eq(&self, other: &Point) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl Point {
    pub fn new(x: f64, y: f64) -> Point {
        Point {
            x: x,
            y: y
        }
    }
}

