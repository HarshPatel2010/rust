
//defining Trait
trait Hello{
    fn say_hi (&self)->String{
        String::from("hi")
    }
    fn say_something(&self)->String;
}

struct Student{}
struct Teacher{}

impl Hello for Student{
    fn say_something(&self)->String{
        String::from("I am a good student")
    }
}
impl Hello for Teacher{
    fn say_hi(&self)->String{
        String::from("I am your new teacher")
    }
    fn say_something(&self)->String{
        String::from("I am not a bad teacher")
    }
}
pub fn main (){
let s:Student= Student{};
    assert_eq!("hi", s.say_hi());
    assert_eq!("I am a good student", s.say_something());
    let t:Teacher = Teacher{};
    assert_eq!("I am your new teacher", t.say_hi());
    assert_eq!("I am not a bad teacher", t.say_something());
    println!("Hello, world!");


    struct Sheep { naked: bool, name: String }

    trait Animal {
        // Associated function signature; `Self` refers to the implementor type.
        fn new(name: String) -> Self;

        // Method signatures; these will return a string.
        fn name(&self) -> String;

        fn noise(&self) -> String;

        // Traits can provide default method definitions.
        fn talk(&self) {
            println!("{} says {}", self.name(), self.noise());
        }
    }

    impl Sheep {
        fn is_naked(&self) -> bool {
            self.naked
        }

        fn shear(&mut self) {
            if self.is_naked() {
                // Implementor methods can use the implementor's trait methods.
                println!("{} is already naked...", self.name());
            } else {
                println!("{} gets a haircut!", self.name);

                self.naked = true;
            }
        }
    }

    // Implement the `Animal` trait for `Sheep`.
    impl Animal for Sheep {
        // `Self` is the implementor type: `Sheep`.
        fn new(name: String) -> Sheep {
            Sheep { name: name, naked: false }
        }

        fn name(&self) -> String {
            self.name.clone()
        }

        fn noise(&self) -> String {
            if self.is_naked() {
                "baaaaah?".to_string()
            } else {
                "baaaaah!".to_string()
            }
        }

        // Default trait methods can be overridden.
        fn talk(&self) {
            // For example, we can add some quiet contemplation.
            println!("{} pauses briefly... {}", self.name, self.noise());
        }
    }

    fn main() {
        // Type annotation is necessary in this case.
        let mut dolly: Sheep = Animal::new("Dolly".to_string());
        // TODO ^ Try removing the type annotations.

        dolly.talk();
        dolly.shear();
        dolly.talk();
    }
    main()

}