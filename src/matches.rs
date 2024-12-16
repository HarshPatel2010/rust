pub enum Coins{
    penny,
    ruppe,
    paisa,
    dollar
}

pub fn main(){

}
pub fn value_in_coin(coin:Coins)->u8{
    match coin{
        Coins::penny=>{
            println!("this is penny");
            1
        },
        Coins::ruppe=>2,
        Coins::paisa=>3,
        Coins::dollar=>4,
    }

}