
// Fill in the blank
pub fn main() {
    let mut n = 0;
    for i in 0..=100 {
        if n == 66 {
            break;
        };
        n += 1;
    }

    assert_eq!(n, 66);

    println!("Success!");


    //continue and break
    let mut n = 0;
    for i in 0..=100 {
        if n != 66 {
            n+=1;
            continue;
        }

        break;
    }

    assert_eq!(n, 66);

    println!("Success! continue");

    let mut count = 0u32;

    println!("Let's count until infinity!");

    // Infinite loop
    loop {
        count += 1;

        if count == 3 {
            println!("three");

            // Skip the rest of this iteration
            continue;
        }

        println!("{}", count);

        if count == 5 {
            println!("OK, that's enough");

            break;
        }
    }

    assert_eq!(count, 5);

    println!("Success!");
}
