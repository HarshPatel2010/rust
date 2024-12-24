
// FILL in the blanks and FIX errors
// 1. Don't use `to_string()`
// 2. Don't add/remove any code line
pub fn main() {
    let mut s: String = String::from("hello, ");
    s.push_str("world");
    s.push('!');

    move_ownership(s.clone());

    assert_eq!(s, "hello, world!");

    println!("Success!");
    // FILL in the blanks
    fn main() {
        let mut s:String = String::from("hello, world");

        let slice1: &str = s.as_str(); // In two ways
        assert_eq!(slice1, "hello, world");

        let slice2:&str = &s[..5];
        assert_eq!(slice2, "hello");

        let slice3: &mut String = &mut s;
        slice3.push('!');
        assert_eq!(slice3, "hello, world!");

        println!("Success!");

        // Question: how many heap allocations are happening here?
        // Your answer:
        fn main() {
            // Create a String type based on `&str`
            // The type of string literals is `&str`
            let s: String = String::from("hello, world!");

            // Create a slice point to String `s`
            let slice: &str = &s;

            // Create a String type based on the recently created slice
            let s: String = slice.to_string();

            assert_eq!(s, "hello, world!");

            println!("Success!2");

            // FILL in the blank and FIX errors
            fn main() {
                let s:String = String::from("hello, 世界");
                let slice1:&str = &s[..1]; //tips: `h` only takes 1 byte in UTF8 format
                assert_eq!(slice1, "h");

                let slice2:&str = &s[7..10]; // Tips: `中`  takes 3 bytes in UTF8 format
                assert_eq!(slice2, "世");

                // Iterate through all chars in s
                for (i, c) in s.chars().enumerate() {
                    if i == 7 {
                        assert_eq!(c, '世')
                    }
                }

                println!("Success! indexing");
            }
            main()
        }
        main()
    }
    main()

}

pub fn move_ownership(s: String) {
    println!("ownership of \"{}\" is moved here!", s)
}