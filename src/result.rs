
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
    }
    main()
}