use std::fmt;

/// A two-dimensional point.
pub struct Point {
    pub x: f32,
    pub y: f32,
}

impl Point {
    ///Creates a new point. 
    /// ```
    /// let result = geometry::point::Point::new(2.0, 3.0);
    /// assert_eq!(result.x, 2.0);
    /// ```
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    /// Calculates the distance to the origin (which is the point (0|0))
    /// ```
    /// let result = geometry::point::Point::new(2.0, 0.0);
    /// let dist = result.distance_to_origin();
    /// assert_eq!(dist, 2.0)
    /// ```
    pub fn distance_to_origin(&self) -> f32 {
        (self.x.powf(2.0) + self.y.powf(2.0)).sqrt()
    }

    /// Calculates the distance to another point. 
    /// ```
    /// let result = geometry::point::Point::new(2.0, 0.0);
    /// let dist = result.distance(&result);
    /// assert_eq!(dist, 0.0)
    /// ```
    pub fn distance(&self, other: &Point) -> f32 {
        ((self.x - other.x).powf(2.0) + (self.y - other.y).powf(2.0)).sqrt()
    }
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}|{})", self.x, self.y)
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
