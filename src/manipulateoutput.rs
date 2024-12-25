use std::fmt;

pub struct Point {
    x: i32,
    y: i32,
}

impl fmt::Display for Point {
    // IMPLEMENT fmt method
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
        write!(f,"The point is ({}, {})",self.x,self.y)
    }
}

pub fn main() {
    let origin:Point = Point { x: 0, y: 0 };
    // FILL in the blanks
    assert_eq!(origin.to_string(), "The point is (0, 0)");
    assert_eq!(format!("{}",origin), "The point is (0, 0)");
    println!("{}",origin);

    println!("Success!");
}