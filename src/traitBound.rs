pub fn main() {
    assert_eq!(sum(1, 2), 3);
    println!("{}", sum(5, 5));
}

// Implement `fn sum` with trait bound in two ways.
pub fn sum<T: std::ops::Add<Output=T>>(x: T, y: T) -> T {
    x + y // x.add(y)
}