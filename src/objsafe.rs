
// Use at least two approaches to make it work.
// DON'T add/remove any code line.
pub trait MyTrait {
    fn f(&self) -> Self;
}

impl MyTrait for u32 {
    fn f(&self) -> Self { 42 }
}

impl MyTrait for String {
    fn f(&self) -> Self { self.clone() }
}

pub fn my_function<T:MyTrait>(x:T)  ->T{
    x.f()
}

pub fn main() {
    my_function(13_u32);
    my_function(String::from("abc"));

    println!("Success2!");

}