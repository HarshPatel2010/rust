
// Fill the blanks
pub enum Direction {
    East,
    West,
    North,
    South,
}

pub fn main() {
    let dire = Direction::South;
    match dire {
        Direction::East => println!("East"),
        Direction::South  => { // Matching South or North here
            println!("South or North");
        },
        Direction::North => println!("North"),
        _ => println!("West"),

    };
    let boolean = true;

    // Fill the blank with a match expression:
    //
    // boolean = true => binary = 1
    // boolean = false =>  binary = 0
    let binary = match boolean {
        true =>1,
        false=>0
    };

    assert_eq!(binary, 1);

    println!("Success!");

    // Fill in the blanks
    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }

    fn main() {
        let msgs = [
            Message::Quit,
            Message::Move{x:1, y:3},
            Message::ChangeColor(255,255,0)
        ];

        for msg in msgs {
            show_message(msg)
        }

        println!("Success!");
    }

    fn show_message(msg: Message) {
        match msg {
            Message::Move{x:a,y:b} => { // match  Message::Move
                assert_eq!(a, 1);
                assert_eq!(b, 3);
                println!("here is the data")
            },
            Message::ChangeColor(_, g, b) => {
                assert_eq!(g, 255);
                assert_eq!(b, 0);
                println!("here is the data2")
            }
            _ => println!("no data in these variants")
        }
    }


        let alphabets = ['a', 'E', 'Z', '0', 'x', '9' , 'Y'];

        // Fill the blank with `matches!` to make the code work
        for ab in alphabets {
            assert!(matches!(ab,'a'..='z'|'A'..='z'|'0'..='9'))
        }

        println!("Success!");

    let mut count = 0;

    let v = vec![MyEnum::Foo,MyEnum::Bar,MyEnum::Foo];
    for e in v {
        if matches!(e,MyEnum::Foo) { // Fix the error by changing only this line
            count += 1;
        }
    }

    assert_eq!(count, 2);

    println!("Success! {}",count);
    enum MyEnum {
        Foo,
        Bar
    }

}