
// FILL in the blanks.
pub trait Draw {
    fn draw(&self) -> String;
}

impl Draw for u8 {
    fn draw(&self) -> String {
        format!("u8: {}", self)
    }
}

impl Draw for f64 {
    fn draw(&self) -> String {
        format!("f64: {}", self)
    }
}

pub fn main() {
    let x:f64 = 1.1f64;
    let y:u8 = 8u8;

    // Draw x.
    draw_with_box(Box::new(x));

    // Draw y.
    draw_with_ref(&y);

    println!("Success!");
}

pub fn draw_with_box(x: Box<dyn Draw>) {
    x.draw();
}

pub fn draw_with_ref(x: &dyn Draw) {
    x.draw();
}