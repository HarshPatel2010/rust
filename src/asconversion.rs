// FIX the errors and FILL in the blank
// DON'T remove any code

pub fn main() {
    let decimal:f32 = 97.123_f32;

    let integer: u8 = decimal as u8;

    let c1: char = decimal as u8 as char;
    let c2:char = integer as char;

    assert_eq!(integer+1, 'b' as u8);

    println!("Success!");
    #[allow(overflowing_literals)]
    fn main() {
        assert_eq!(u8::MAX, 255);
        // The max of `u8` is 255 as shown above.
        // so the below code will cause an overflow error: literal out of range for `u8`.
        // PLEASE looking for clues within compile errors to FIX it.
        // DON'T modify any code in main.

        let v:u8 = 1000 as u8;
       println!("{:#010b}", v);

        println!("Success!");
    }
    main()
}