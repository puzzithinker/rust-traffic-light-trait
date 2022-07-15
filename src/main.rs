// Create an `enum` to classify a Traffic Light
enum TrafficLight {
    Red,
    Green,
    Yellow,
}

impl TrafficLight {
    //return light time function
     fn return_light_time(&self) -> u8 {
        match &*self{
            TrafficLight::Red => 60,
            TrafficLight::Green => 40,
            TrafficLight::Yellow => 20,
        }
    }
}

//main method

fn main() {
    let light = TrafficLight::Yellow;
    println!("light is {}",light.return_light_time());

}