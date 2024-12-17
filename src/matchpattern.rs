
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
}