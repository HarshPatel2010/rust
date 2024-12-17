
pub fn main(){
    let config_max:Option<u8>= Some(20_u8);
    if let Some(max) = config_max{
        println!("The config max is {}",max);
    }


    // Fix the errors without adding or removing lines

        let names = [String::from("liming"),String::from("hanmeimei")];
        for name in &names {
            println!("the name is {}",name);
        }

        println!("{:?}", names);

        let numbers = [1, 2, 3];
        // The elements in numbers are Copy，so there is no move here
        for n in numbers {
            println!("the number is {}",n)
        }

        println!("{:?}", numbers);

}