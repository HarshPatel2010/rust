pub fn main() {
    let o = Some(7);

    // Remove the whole `match` block, using `if let` instead
    //  match o {
    //     Some(i) => {
    //        println!("This is a really long string and `{:?}`", i);

    //        println!("Success!");
    //     }
    //    _ => {}
    //   };


    if let Some(i) = o{
        println!("already exist")
    }

    // Fill in the blank
    enum Foo {
        Bar(u8)
    }


        let a = Foo::Bar(1);

        if let Foo::Bar(i)  = a {
            println!("foobar holds the value: {}", i);

            println!("Success!");
        }



    enum Fooo {
        Bar,
        Baz,
        Qux(u32)
    }

        let a = Fooo::Qux(10);

        // Remove the codes below, using `match` instead
        //  if let Foo::Bar = a {
        //    println!("match foo::bar")
        //    } else if let Foo::Baz = a {
        //      println!("match foo::baz")
        //} else {
        //  println!("match others")
        //    }

        match a {
            Fooo::Bar => println!("the bar"),
            Fooo::Baz => println!("the baz"),
            Fooo::Qux(value)=>println!("qux value: {}", value),
        };

    let age = Some(30);
    if let Some(age) = age {
        // Create a new variable with the same name as previous `age`

        assert_eq!(age, 30);
    } // The new variable `age` goes out of scope here

    match age {
        // Match can also introduce a new shadowed variable
        Some(age) =>  println!("age is a new variable, it's value is {}",age),
        _ => ()
    }


}