
pub struct Structure(i32);

pub struct Deep(Structure);

impl std::fmt::Debug for Deep{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result{
        write!(f,"{}",self.0.0)
    }
}


pub fn main() {
    // The problem with `derive` is there is no control over how
    // the results look. What if I want this to just show a `7`?

    /* Make it print: Now 7 will print! */
    println!("Now {:?} will print!", Deep(Structure(7)));
}