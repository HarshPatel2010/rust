
// Fill in the blanks to make it work
pub struct A;          // Concrete type `A`.
pub struct S(A);       // Concrete type `S`.
pub struct SGen<T>(T); // Generic type `SGen`.

fn reg_fn(_s: S) {}

fn gen_spec_t(_s: SGen<A>) {}

fn gen_spec_i32(_s: SGen<i32>) {}

fn generic<T>(_s: SGen<T>) {}
pub fn sum<T: std::ops::Add<Output = T>+ std::fmt::Display>(a:T,b:T)->T{
    let  result ;
    result = a+b;
    println!("{}", result);
    result
}

// Implement struct Point to make it
pub struct Point<T>{
    x:T,
    y:T,
}
pub fn main() {
    // Using the non-generic functions
    reg_fn(S(A));          // Concrete type.
    gen_spec_t(SGen(A));   // Implicitly specified type parameter `A`.
    gen_spec_i32(SGen(7)); // Implicitly specified type parameter `i32`.

    // Explicitly specified type parameter `char` to `generic()`.
    generic::<char>(SGen('A'));

    // Implicitly specified type parameter `char` to `generic()`.
    generic(SGen(7.7));

    println!("Success!");
    assert_eq!(5, sum(2i8, 3i8));
    assert_eq!(50, sum(20, 30));
    assert_eq!(2.46, sum(1.23, 1.23));

    println!("Success!");
    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.0 };

    println!("Success!");
    xmain();
    xxmain();
    mmain();
}


// Modify this struct to make the code work
struct Points<T,U> {
    x: T,
    y: U,
}

pub fn xmain() {
    // DON'T modify this code.
    let p = Points{x: 5, y : "hello".to_string()};

    println!("Success2!");
}


// Add generic for Val to make the code work, DON'T modify the code in `main`.
pub struct Val<T> {
    val: T,
}

impl <T> Val <T>{
    fn value(&self) -> &T {
        &self.val
    }
}


pub fn xxmain() {
    let x = Val{ val: 3.0 };
    let y = Val{ val: "hello".to_string()};
    println!("{}, {}", x.value(), y.value());
}


//methods
pub struct xPoint<T, U> {
    x: T,
    y: U,
}

impl<T, U> xPoint<T, U> {
    // Implement mixup to make it work, DON'T modify other code.
    fn mixup<V,W>(self,other:xPoint<V,W>)->xPoint<T,W>{
        xPoint{
            x:self.x,
            y:other.y
        }
    }
}

pub fn mmain() {
    let p1 = xPoint { x: 5, y: 10 };
    let p2 = xPoint { x: "Hello", y: '中'};

    let p3:xPoint<i32,char> = p1.mixup(p2);


    assert_eq!(p3.x, 5);
    assert_eq!(p3.y, '中');

    println!("Success!!!!!");

    // Fix the errors to make the code work.
    struct Point<T> {
        x: T,
        y: T,
    }

    impl Point<f64> {
        fn distance_from_origin(&self) -> f64 {
            (self.x.powi(2) + self.y.powi(2)).sqrt()
        }
    }

    fn main() {
        let p = Point{x: 5.0, y: 10.0};
        println!("{}",p.distance_from_origin());
    }
    main()
}
