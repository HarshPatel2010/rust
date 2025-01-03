
// FILL in the blanks and FIX the errors
use std::num::ParseIntError;

pub fn multiply(n1_str: &str, n2_str: &str) -> Result<i32,ParseIntError> {
    let n1:Result<i32,ParseIntError> = n1_str.parse::<i32>();
    let n2:Result<i32,ParseIntError> = n2_str.parse::<i32>();
    Ok(n1.unwrap() * n2.unwrap())
}

pub fn main() {
    let result:Result<i32,ParseIntError> = multiply("10", "2");
    assert_eq!(result, Ok(20));

    let result = multiply("4", "2");
    assert_eq!(result.unwrap(), 8);


    use std::num::ParseIntError;

    // IMPLEMENT multiply with ?
    // DON'T use unwrap here
    fn multiply(n1_str: &str, n2_str: &str) -> Result<i32,ParseIntError> {
       let n1:i32 = n1_str.parse::<i32>()?;
        let n2:i32 = n2_str.parse::<i32>()?;
        Ok(n1*n2)
    }

    fn main() {
        assert_eq!(multiply("3", "4").unwrap(), 12);
        println!("Success!, {}", multiply("3", "4").unwrap());

        use std::fs::File;
        use std::io::{self, Read};

        fn read_file1() -> Result<String, io::Error> {
            let f:Result<File,io::Error> = File::open("hello.txt");
            let mut f :File= match f {
                Ok(file) => file,
                Err(e) => return Err(e),
            };

            let mut s :String= String::new();
            match f.read_to_string(&mut s) {
                Ok(_) => Ok(s),
                Err(e) => Err(e),
            }
        }

        // FILL in the blanks with one code line
        // DON'T change any code lines
        fn read_file2() -> Result<String, io::Error> {
            let mut s :String= String::new();

            File::open("hello.txt")?.read_to_string(&mut s)?;
            Ok(s)
        }

        fn main() {
            assert_eq!(read_file1().unwrap_err().to_string(), read_file2().unwrap_err().to_string());
            println!("Success!");
            use std::num::ParseIntError;

            // FILL in the blank in two ways: map, and then
            fn add_two(n_str: &str) -> Result<i32, ParseIntError> {
              //  n_str.parse::<i32>().map(|n| n + 2)
                n_str.parse::<i32>().and_then(|n| Ok(n + 2))

            }

            fn main() {
                assert_eq!(add_two("4").unwrap(), 6);

                println!("Success! {}",add_two("4").unwrap());
                use std::num::ParseIntError;

                // With the return type rewritten, we use pattern matching without `unwrap()`.
                // But it's so Verbose...
                fn multiply(n1_str: &str, n2_str: &str) -> Result<i32, ParseIntError> {
                    match n1_str.parse::<i32>() {
                        Ok(n1)  => {
                            match n2_str.parse::<i32>() {
                                Ok(n2)  => {
                                    Ok(n1 * n2)
                                },
                                Err(e) => Err(e),
                            }
                        },
                        Err(e) => Err(e),
                    }
                }

                // Rewriting `multiply` to make it succinct
                // You should use BOTH of  `and_then` and `map` here.
                fn multiply1(n1_str: &str, n2_str: &str) -> Result<i32, ParseIntError> {
                    // IMPLEMENT...
                    n1_str.parse::<i32>().and_then(|n1:i32| n2_str.parse::<i32>().map(|n2:i32| n1 * n2) )
                }

                fn print(result: Result<i32, ParseIntError>) {
                    match result {
                        Ok(n)  => println!("n is {}", n),
                        Err(e) => println!("Error: {}", e),
                    }
                }

                fn main() {
                    // This still presents a reasonable answer.
                    let twenty:Result<i32,ParseIntError> = multiply1("10", "2");
                    print(twenty);

                    // The following now provides a much more helpful error message.
                    let tt:Result<i32,ParseIntError> = multiply("t", "2");
                    print(tt);

                    println!("Success!");
                }
                main()
            }
            main()
        }
        main()
    }
    main()
}