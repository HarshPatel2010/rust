
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

    trait Foo {
        fn method(&self) -> String;
    }

    impl Foo for u8 {
        fn method(&self) -> String { format!("u8: {}", *self) }
    }

    impl Foo for String {
        fn method(&self) -> String { format!("string: {}", *self) }
    }

    // IMPLEMENT below with generics.
    fn static_dispatch<T:Foo>(a:T){
        println!("{}", a.method());
    }

    // Implement below with trait objects.
    fn dynamic_dispatch(a:&dyn Foo){
        println!("{}", a.method());
    }

    fn main() {
        let x = 5u8;
        let y = "Hello".to_string();

        static_dispatch(x);
        dynamic_dispatch(&y);

        println!("Success!");
    }
    main()
}

pub fn draw_with_box(x: Box<dyn Draw>) {
    x.draw();
}

pub fn draw_with_ref(x: &dyn Draw) {
    x.draw();
}