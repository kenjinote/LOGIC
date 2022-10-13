pub struct Node {
    x: f64,
    y: f64,
    width: f64,
    height: f64,

}

use winapi::um::wingdi::TextOutW;
use crate::utility::encode;
use winapi::shared::windef::{HDC};

impl Node {
    pub fn new(x: f64, y: f64, width: f64, height: f64) -> Node {
        Node {
            x: x,
            y: y,
            width: width,
            height: height,
        }
    }

    pub fn draw(&self, hdc : HDC) {
        unsafe {
            TextOutW(hdc, self.x as i32, self.y as i32, encode("Hello").as_ptr(), 5);
        }
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