use crate::point::Point;
use crate::traits;

pub struct Rectangle {
    center: Point,
    length: f32,
    width: f32,
}

impl Rectangle {
    pub fn new(center: Point, length: f32, width: f32) -> Self {
        Rectangle {
            center,
            length,
            width,
        }
    }
}

impl traits::Shape<f32> for Rectangle {
    fn area(&self) -> f32 {
        self.length * self.width
    }

    fn circumference(&self) -> f32 {
        2.0 * self.length + 2.0 * self.width
    }
}
