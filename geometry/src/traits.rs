pub trait Shape<T> {
    fn area(&self) -> T;
    fn circumference(&self) -> T;
}
