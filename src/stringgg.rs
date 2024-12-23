
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
    }
    main()
}

pub fn move_ownership(s: String) {
    println!("ownership of \"{}\" is moved here!", s)
}