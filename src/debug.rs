
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

    /* Make it work */
    use std::fmt;

    struct List(Vec<i32>);

    impl fmt::Display for List {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            // Extract the value using tuple indexing,
            // and create a reference to `vec`.
            let vec:&Vec<i32> = &self.0;

            write!(f, "[")?;

            // Iterate over `v` in `vec` while enumerating the iteration
            // count in `count`.
            for (count, v) in vec.iter().enumerate() {
                // For every element except the first, add a comma.
                // Use the ? operator to return on errors.
                if count != 0 { write!(f, ", ")?; }
                write!(f, "{}: {}",count, v)?;
            }

            // Close the opened bracket and return a fmt::Result value.
            write!(f, "]")
        }
    }

    fn main() {
        let v = List(vec![1, 2, 3]);
        assert_eq!(format!("{}",v), "[0: 1, 1: 2, 2: 3]");
        println!("Success! {}",format!("{}", v));
    }
    main()
}