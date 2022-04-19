pub struct Point {
    pub x: f32,
    pub y: f32,
}

impl Point {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    pub fn distance_to_origin(&self) -> f32 {
        (self.x.powf(2.0) + self.y.powf(2.0)).sqrt()
    }

    pub fn distance(&self, other: &Point) -> f32 {
        ((self.x-other.x).powf(2.0) + (self.y-other.y).powf(2.0)).sqrt()
    }
}

#[cfg(test)]
mod point_tests {

    use super::*;

    #[test]
    fn dist_to_origin() {
        let point = Point::new(1.0, 0.0);
        approx::assert_relative_eq!(point.distance_to_origin(), 1.0);
    }

    #[test]
    fn dist_to_same_point() {
        let point = Point::new(1.0, 0.0);
        approx::assert_relative_eq!(point.distance(&point), 0.0);
    }
}
