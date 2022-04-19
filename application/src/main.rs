use geometry::traits::Shape;

fn main() {
    let point = geometry::point::Point::new(1.0, 2.0);
    println!("Distance to origin: {}", point.distance_to_origin());
    println!("Point {}", point);
    let my_rect = geometry::shapes::Rectangle::new(point, 4.0, 3.0);
    println!("Area of rectangle: {}", my_rect.area());
}
