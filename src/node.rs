pub struct Node {
    x: f64,
    y: f64,
    width: f64,
    height: f64,

}

impl Node {
    pub fn new(x: f64, y: f64, width: f64, height: f64) -> Node {
        Node {
            x: x,
            y: y,
            width: width,
            height: height,
        }
    }

    pub fn draw() {
    }

    fn contains(&self, x: f64, y: f64) -> bool {
        if x >= self.x && x <= self.x + self.width && y >= self.y && y <= self.y + self.height {
            return true;
        }
        return false;
    }

    fn intersects(&self, range: &Node) -> bool {
        if range.x > self.x + self.width || range.x + range.width < self.x || range.y > self.y + self.height || range.y + range.height < self.y {
            return false;
        }
        return true;
    }
}