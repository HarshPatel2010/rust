#[derive(Debug)]
pub enum IpAdd{
    v4(String),
    v6(String),
}
pub fn main() {
    let home :IpAdd=IpAdd::v4(String::from("127.0.0.1"));
    rout(home);
}
fn rout(ip:IpAdd) {
    println!("ip address is {:?}",ip)
}