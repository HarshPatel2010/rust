// `print_refs` takes two references to `i32` which have different
// lifetimes `'a` and `'b`. These two lifetimes must both be at
// least as long as the function `print_refs`.
pub fn print_refs<'a, 'b>(x: &'a i32, y: &'b i32) {
    println!("x is {} and y is {}", x, y);
}

/* Make it work */
// A function which takes no arguments, but has a lifetime parameter `'a`.
pub fn failed_borrow() {
    let _x:i32 = 12;

    // ERROR: `_x` does not live long enough
    let y: &i32 = &_x;
    // Attempting to use the lifetime `'a` as an explicit type annotation
    // inside the function will fail because the lifetime of `&_x` is shorter
    // than `'a` . A short lifetime cannot be coerced into a longer one.
}

pub fn main() {
    let (four, nine) = (4, 9);

    // Borrows (`&`) of both variables are passed into the function.
    print_refs(&four, &nine);
    // Any input which is borrowed must outlive the borrower.
    // In other words, the lifetime of `four` and `nine` must
    // be longer than that of `print_refs`.

    failed_borrow();
    // `failed_borrow` contains no references to force `'a` to be
    // longer than the lifetime of the function, but `'a` is longer.
    // Because the lifetime is never constrained, it defaults to `'static`.

    /* Make it work by adding proper lifetime annotation */

    // A type `Borrowed` which houses a reference to an
    // `i32`. The reference to `i32` must outlive `Borrowed`.
    #[derive(Debug)]
    struct Borrowed<'a>(&'a i32);

    // Similarly, both references here must outlive this structure.
    #[derive(Debug)]
    struct NamedBorrowed<'a> {
        x: &'a i32,
        y: &'a i32,
    }

    // An enum which is either an `i32` or a reference to one.
    #[derive(Debug)]
    enum Either<'a> {
        Num(i32),
        Ref(&'a i32),
    }

    fn main() {
        let x = 18;
        let y = 15;

        let single = Borrowed(&x);
        let double = NamedBorrowed { x: &x, y: &y };
        let reference = Either::Ref(&x);
        let number    = Either::Num(y);

        println!("x is borrowed in {:?}", single);
        println!("x and y are borrowed in {:?}", double);
        println!("x is borrowed in {:?}", reference);
        println!("y is *not* borrowed in {:?}", number);
        /* Make it work */

        #[derive(Debug)]
        struct NoCopyType {}

        #[derive(Debug)]
        struct Example<'a, 'b> {
            a: &'a u32,
            b: &'b NoCopyType
        }

        fn main()
        {
            /* 'a tied to fn-main stackframe */
            let var_a = 35;
            let example: Example;

            {
                /* Lifetime 'b tied to new stackframe/scope */
                let var_b = NoCopyType {};

                /* fixme */
                example = Example { a: &var_a, b: &var_b };
                println!("(Success!) {:?}", example);
                struct Owner(i32);

                impl Owner {
                    // Annotate lifetimes as in a standalone function.
                    fn add_one<'a>(&'a mut self) { self.0 += 1; }
                    fn print<'a>(&'a self) {
                        println!("`print`: {}", self.0);
                    }
                }

                fn main() {
                    let mut owner = Owner(18);

                    owner.add_one();
                    owner.print();
                    /* Make it work by adding proper lifetime annotations */
                    struct ImportantExcerpt<'a> {
                        part: &'a str,
                    }

                    impl ImportantExcerpt<'_> {
                        fn level<'a>(&'a self) ->  i32 {
                        3
                        }
                    }

                    fn main() {
                        fn main() {
                            let static_string = "I'm in read-only memory";

                            {
                                // Make a `string` literal and print it:
                                println!("static_string: {}", static_string);

                                // When `static_string` goes out of scope, the reference
                                // can no longer be used, but the data remains in the binary.
                            }

                            println!("static_string reference remains alive: {}", static_string);
                            fn main() {
                                // Increment via closures and functions.
                                fn function(i: i32) -> i32 { i + 1 }

                                // Closures are anonymous, here we are binding them to references
                                //
                                // These nameless functions are assigned to appropriately named variables.
                                let closure_annotated = |i: i32| -> i32 { i + 1 };
                                let closure_inferred  = |i     |          i + 1  ;

                                let i = 1;
                                // Call the function and closures.
                                println!("function: {}", function(i));
                                println!("closure_annotated: {}", closure_annotated(i));
                                println!("closure_inferred: {}", closure_inferred(i));

                                // A closure taking no arguments which returns an `i32`.
                                // The return type is inferred.
                                let one = || 1;
                                println!("closure returning one: {}", one());
                                fn main() {
                                    let example_closure = |x| x;

                                    let s = example_closure(String::from("hello"));

                                    /* Make it work, only change the following line */
                                    let n = example_closure(5.to_string());
                                    fn main() {
                                        let x = 1;
                                        let closure = |val| val + x;
                                        assert_eq!(closure(2), 3);
                                    }
                                    main()
                                }
                                main()

                            }
                            main()
                        }
                        main()
                    }
                    main()
                }
                main()
            }

        }
        main()
    }
    main()
}