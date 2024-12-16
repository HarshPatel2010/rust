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