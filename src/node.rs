use winapi::um::wingdi::{TextOutW, Rectangle,SelectObject,GetStockObject,WHITE_BRUSH,DC_BRUSH,CreatePen,PS_SOLID,DC_PEN,DeleteObject,RGB};
use crate::utility::encode;
use winapi::shared::windef::{HDC, HPEN, HGDIOBJ};

pub struct Node {
    x: f64,
    y: f64,
    width: f64,
    height: f64,
    born: u32,
    death: u32,
    selected: bool,
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
            selected: false,
        }
    }

    pub fn select(&mut self) {
        self.selected = true;
    }

    pub fn unselect(&mut self) {
        self.selected = false;
    }

    pub fn isselected(&mut self) -> bool{
        self.selected
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
            let pen : HPEN =
            if self.selected {
                CreatePen(PS_SOLID as i32, 1, RGB(255, 0, 0))
            } else {
                CreatePen(PS_SOLID as i32, 1, RGB(0, 0, 0))
            };
            let old_pen = SelectObject(hdc, pen as HGDIOBJ);
            Rectangle(hdc, self.x as i32, self.y as i32, (self.x + self.width)  as i32, (self.y + self.height) as i32);
            SelectObject(hdc, old_pen as HGDIOBJ);
            DeleteObject(pen as HGDIOBJ);
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