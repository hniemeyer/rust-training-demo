struct Point {
    x: f32,
    y: f32,
}

impl Point {
    fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    fn distance_to_origin(&self) -> f32 {
        (self.x.powf(2.0) + self.y.powf(2.0)).sqrt()
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
}
