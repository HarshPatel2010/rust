
// Fill in the blanks to make it work
pub struct A;          // Concrete type `A`.
pub struct S(A);       // Concrete type `S`.
pub struct SGen<T>(T); // Generic type `SGen`.

fn reg_fn(_s: S) {}

fn gen_spec_t(_s: SGen<A>) {}

fn gen_spec_i32(_s: SGen<i32>) {}

fn generic<T>(_s: SGen<T>) {}

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
}