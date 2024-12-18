
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
}