/// The shape trait specifies that a shape should
/// have an area and a circumference. The genereic
/// type T is the return type of area and circumference
/// calculation.
pub trait Shape<T> {
    /// Calculate the area of the geometric shape.
    fn area(&self) -> T;
    /// Calculates the circumference of the geometric shape. 
    fn circumference(&self) -> T;
}
