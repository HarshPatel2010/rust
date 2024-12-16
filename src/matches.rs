#[derive(Debug)]
pub enum UsState{
    Alabama,
    NewYork
}
#[derive(Debug)]
pub enum Coins{
    penny,
    ruppe,
    paisa,
    dollar(UsState)
}


pub fn main(){
let coin:Coins = Coins::dollar(UsState::Alabama);
    println!("Penny's value is {}",value_in_coin(coin));
   // println!("sum is {}",add(50,Some(90)));
    println!("sum is {}",add(50,Some(90)));

    let dice_roll:i32 = 1;
    match dice_roll{
        3 => println!("congrats you got hat"),
        6 => println!("Bad luck your hat has been removed"),
        //  other=>println!("dice moved to {}",other)

        _ => println!("Can't do that"),
    }
}
fn add(num:i32,num2:Option<i32>)->i32{
    match num2{
        Some(i)=>num+i,
        None=>num,
    }
}
pub fn value_in_coin(coin:Coins)->u8{
    match coin{
        Coins::penny=>{
            println!("this is penny");
            1
        },
        Coins::ruppe=>2,
        Coins::paisa=>3,
        Coins::dollar(UsState::NewYork)=>{
            println!("this is dollar");
            20
        }
        Coins::dollar(state)=>{
            println!("got Dollar state {:?}",state);
            25
        },
    }

}