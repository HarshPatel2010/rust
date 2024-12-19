
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

}