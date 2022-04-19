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

    pub fn move_center(&mut self, point: Point) {
        self.center = point;
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

#[cfg(test)]
mod rectangle_tests {
    use crate::traits::Shape;

    use super::*;

    #[test]
    fn correct_area() {
        let my_rect = Rectangle::new(Point::new(0.0, 0.0), 2.0, 3.0);
        approx::assert_relative_eq!(my_rect.area(), 6.0);
    }

    #[test]
    fn correct_circumference() {
        let my_rect = Rectangle::new(Point::new(0.0, 0.0), 2.0, 3.0);
        approx::assert_relative_eq!(my_rect.circumference(), 10.0);
    }
}
