use winapi::um::wingdi::TextOutW;
use crate::utility::encode;
use winapi::shared::windef::{HDC};

pub struct Node {
    x: f64,
    y: f64,
    width: f64,
    height: f64,
    born: u32,
    death: u32,
}

impl Node {
    pub fn new(x: f64, y: f64, width: f64, height: f64) -> Node {
        Node {
            x: x,
            y: y,
            width: width,
            height: height,
            born: 0,
            death: u32::MAX,
        }
    }

    pub fn kill(&mut self, generation: u32) {
        self.death = generation;
    }

    pub fn is_alive(&self, generation: u32) -> bool {
        self.born <= generation && generation <= self.death
    }

    pub fn hit_test(&self, x: f64, y: f64, generation: u32) -> bool {

        self.is_alive(generation) &&  x >= self.x && x <= self.x + self.width && y >= self.y && y <= self.y + self.height
    }

    pub fn draw(&self, hdc : HDC) {
        unsafe {
            let text = self.born.to_string();
            TextOutW(hdc, self.x as i32, self.y as i32, encode(&text).as_ptr(), text.len() as i32);
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