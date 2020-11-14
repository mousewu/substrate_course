
fn main() {
    let light = TrafficLight::Red;
    let time_ = light.get_time();
    println!("The time for {:?} light is {} seconds!", light, time_);
}

#[derive(Debug)]
pub enum TrafficLight{
    Green,
    Red,
    Yellow
}

trait Time{
    fn get_time(&self) -> u8;
}

impl Time for TrafficLight{
    fn get_time(&self) -> u8{
        match &self {
            TrafficLight::Green => 40,
            TrafficLight::Red => 20,
            TrafficLight::Yellow => 10
        }
    }
}
