use winapi::shared::windef::{HDC};
use crate::node::Node;

pub struct NodeList {
    list: Vec<Node>,
    current_generation: u32,
}

impl NodeList {
    pub const fn new() -> NodeList {
        NodeList {
            list: Vec::new(),
            current_generation: 0,
        }
    }

    pub fn add(&mut self, node: Node) {
        self.list.push(node);
    }

    pub fn remove(&mut self, index: usize) {
        self.list.remove(index);
    }

    pub fn get(&self, index: usize) -> &Node {
        &self.list[index]
    }

    pub fn get_mut(&mut self, index: usize) -> &mut Node {
        &mut self.list[index]
    }

    pub fn len(&self) -> usize {
        self.list.len()
    }

    pub fn clear(&mut self) {
        self.list.clear();
    }

    pub fn hit_test(&self, x: f64, y: f64) -> Option<usize> {
        for i in 0..self.list.len() {
            if self.list[i].hit_test(x, y, self.current_generation) {
                return Some(i);
            }
        }
        return None;
    }

    pub fn draw(&self, hdc : HDC) {
        for i in 0..self.list.len() {
            self.list[i].draw(hdc);
        }
    }
}