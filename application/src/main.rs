fn main() {
    let point = geometry::point::Point::new(1.0, 2.0);
    println!("Distance to origin: {}", point.distance_to_origin());
    println!("Point {}", point);
}
