pub fn main() {
    // impl From<bool> for i32
    let i1: i32 = false.into();
    let i2: i32 = i32::from(false);
    assert_eq!(i1, i2);
    assert_eq!(i1, 0);

    // FIX the error in two ways
    /* 1. use a similar type which `impl From<char>`, maybe you
    should check the docs mentioned above to find the answer */
    // 2. a keyword from the last chapter
    let i3: u32 = 'a'.into();

    // FIX the error in two ways
    let s: String =String::from('a');

    println!("Success!3");
    // From is now included in `std::prelude`, so there is no need to introduce it into the current scope
    // use std::convert::From;

    #[derive(Debug)]
    struct Number {
        value: i32,
    }

    impl From<i32> for Number {
        // IMPLEMENT `from` method]
        fn from(value:i32)->Self{
            Self{value}
        }
    }

    // FILL in the blanks
    fn main() {
        let num:Number = Number::from(30);
        assert_eq!(num.value, 30);

        let num: Number = 30.into();
        assert_eq!(num.value, 30);

        println!("Success!");

        //File --------------------------
        use std::fs;
        use std::io;
        use std::num;

        enum CliError {
            IoError(io::Error),
            ParseError(num::ParseIntError),
        }

        impl From<io::Error> for CliError {
            // IMPLEMENT from method
            fn from(e:io::Error)->Self{
              CliError::IoError(e)
            }
        }

        impl From<num::ParseIntError> for CliError {
            // IMPLEMENT from method
            fn from(e:num::ParseIntError)->Self{
                CliError::ParseError(e)
            }
        }

        fn open_and_parse_file(file_name: &str) -> Result<i32, CliError> {
            // ? automatically converts io::Error to CliError
            let contents:String = fs::read_to_string(&file_name)?;
            // num::ParseIntError -> CliError
            let num: i32 = contents.trim().parse()?;
            Ok(num)
        }

        fn main() {
            println!("Success!---");
        }
        main()
    }
    main()
}