
#[derive(Debug)]
pub enum TrafficLightColor {
    Red,
    Yellow,
    Green,
}

// Implement TrafficLightColor with a method.
impl TrafficLightColor {
    fn color(&self)->String{
        match self{
            TrafficLightColor::Red => "red".to_string(),
            TrafficLightColor::Yellow => "yellow".to_string(),
            TrafficLightColor::Green=>"green".to_string()
        }
    }
}

pub fn main() {
    let c = TrafficLightColor::Green;

    assert_eq!(c.color(), "green");

    println!("{:?}",c);
}